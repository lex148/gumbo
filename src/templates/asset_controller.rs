use crate::change::Change;
use crate::errors::Result;

pub(crate) fn write_template() -> Result<Vec<Change>> {
    Ok(vec![Change::new(
        "./src/controllers/assets_controller.rs",
        CODE,
    )?
    .add_parent_mod()])
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
