use super::ensure_directory_exists;
use super::modrs::append_module;
use super::TemplateError;
use std::fs::File;
use std::io::Write;
use std::path::Path;

pub(crate) fn write_template(root_path: &Path) -> Result<(), TemplateError> {
    let mut path = root_path.to_path_buf();
    path.push("./src/controllers/assets_controller.rs");
    ensure_directory_exists(&path)?;
    append_module(root_path, "./src/controllers/mod.rs", "assets_controller")?;
    let mut file = File::options()
        .write(true)
        .truncate(true)
        .create(true)
        .open(&path)?;

    file.write_all(CODE.as_bytes())?;

    Ok(())
}

static CODE: &str = r#"use actix_web::{get, HttpResponse};

#[get("/app.css")]
async fn styles() -> HttpResponse {
    let content = include_str!(concat!(env!("OUT_DIR"), "/app.css"));
    HttpResponse::Ok()
        .content_type("text/css; charset=utf-8")
        .body(content)
}
"#;
