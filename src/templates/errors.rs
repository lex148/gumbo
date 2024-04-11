use super::{ensure_directory_exists, TemplateError};
use std::fs::File;
use std::io::Write;
use std::path::Path;

pub(crate) fn write_template(root_path: &Path) -> Result<(), TemplateError> {
    let mut path = root_path.to_path_buf();
    path.push("./src/errors.rs");
    ensure_directory_exists(&path)?;
    let mut file = File::options()
        .write(true)
        .truncate(true)
        .create(true)
        .open(&path)?;

    file.write_all(CODE.as_bytes())?;

    Ok(())
}

static CODE: &str = r#"use actix_web::{
    http::{header::ContentType, StatusCode},
    HttpResponse, ResponseError,
};
use thiserror::Error;
use welds::WeldsError;
pub type Result<T> = std::result::Result<T, ServerError>;

#[derive(Debug, Error)]
pub enum ServerError {
    #[error("A database Error occured")]
    DatabaseError(WeldsError),
}

// How the server should Response to an error in the system
impl ResponseError for ServerError {
    #[cfg(debug_assertions)]
    fn error_response(&self) -> HttpResponse {
        let error = format!("ERROR: {:?}", self);
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(error)
    }

    #[cfg(not(debug_assertions))]
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body("")
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            ServerError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl From<welds::WeldsError> for ServerError {
    fn from(inner: welds::WeldsError) -> Self {
        log::error!("{:?}", inner);
        ServerError::DatabaseError(inner)
    }
}
"#;
