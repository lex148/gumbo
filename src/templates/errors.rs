use crate::change::Change;
use crate::errors::Result;

pub(crate) fn write_template() -> Result<Vec<Change>> {
    let c = Change::new("./src/errors.rs", CODE)?;
    Ok(vec![c])
}

static CODE: &str = r##"
use actix_web::{
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
    #[error("Invalid Data")]
    InvalidData,
    #[error("Resource Not Found")]
    ResourceNotFound,
    #[error("OAuth provider {0}: {1}")]
    OAuth(String, String),
}

pub(crate) fn oauth_error<P, E>(provider: P, error: E) -> ServerError
where
    P: Into<String>,
    E: Into<String>,
{
    let p: String = provider.into();
    let err: String = error.into();
    ServerError::OAuth(p, err)
}

// How the server should Response to an error in the system
impl ResponseError for ServerError {
    #[cfg(debug_assertions)]
    fn error_response(&self) -> HttpResponse {
        let error = format!("ERROR: {:?}", self);
        log::error!("{}", error);
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(error)
    }

    #[cfg(not(debug_assertions))]
    fn error_response(&self) -> HttpResponse {
        let error = format!("ERROR: {:?}", self);
        log::error!("{}", error);
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body("")
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            ServerError::OAuth(_, _) => StatusCode::INTERNAL_SERVER_ERROR,
            ServerError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ServerError::InvalidData => StatusCode::UNPROCESSABLE_ENTITY,
            ServerError::ResourceNotFound => StatusCode::NOT_FOUND,
        }
    }
}

impl From<welds::WeldsError> for ServerError {
    fn from(inner: welds::WeldsError) -> Self {
        ServerError::DatabaseError(inner)
    }
}

impl From<oauth2::url::ParseError> for ServerError {
    fn from(inner: oauth2::url::ParseError) -> Self {
        let inner_err = format!("{:?}", inner);
        ServerError::OAuth("???".to_owned(), inner_err)
    }
}

"##;
