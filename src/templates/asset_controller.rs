use super::modrs::append_module;
use super::TemplateError;
use super::{ensure_directory_exists, touch};
use std::fs::File;
use std::io::Write;
use std::path::Path;

pub(crate) fn write_template(root_path: &Path) -> Result<(), TemplateError> {
    let mut path = root_path.to_path_buf();
    path.push("./src/controllers/assets_controller.rs");
    ensure_directory_exists(&path)?;
    append_module(root_path, "./src/controllers/mod.rs", "assets_controller")?;
    let mut file = File::create(&path)?;
    file.write_all(CODE.as_bytes())?;

    let logo = include_bytes!("./gumbo.webp");
    let mut assets_logo = root_path.to_path_buf();
    assets_logo.push("./src/assets/gumbo.webp");
    ensure_directory_exists(&assets_logo)?;
    let mut file = File::create(&assets_logo)?;
    file.write_all(logo)?;

    touch(root_path, "./src/assets/.gitkeep")?;

    Ok(())
}

static CODE: &str = r#"use actix_web::{get, HttpResponse};

// Files in the src/assets/** directory are served using actix_files
// You can embed/serve files here too if you wish.
//

#[get("/app.css")]
async fn styles() -> HttpResponse {
    let content = include_str!(concat!(env!("OUT_DIR"), "/app.css"));
    HttpResponse::Ok()
        .content_type("text/css; charset=utf-8")
        .body(content)
}
"#;
