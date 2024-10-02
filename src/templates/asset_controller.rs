use crate::change::Change;
use crate::errors::Result;

pub(crate) fn write_template() -> Result<Vec<Change>> {
    Ok(vec![Change::new(
        "./src/controllers/assets_controller.rs",
        CODE,
    )?
    .add_parent_mod()])
}

static CODE: &str = r#"
use crate::errors::{Result, ServerError::ResourceNotFound};
use actix_web::web::Path;
use actix_web::{get, HttpResponse};
use gumbo_lib::javascript::JsFile;

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


#[get("/assets/js/{filename}-{hash}.js")]
async fn javascript(path: Path<(String, String)>) -> Result<HttpResponse> {
    // get the contents of the JS file
    let (filename, hash) = path.into_inner();
    let file = JsFile::new(&filename).or(Err(ResourceNotFound))?;

    // verify the contents match the hash
    file.verify_hash(&hash).or(Err(ResourceNotFound))?;

    // returns the contents of the JS file (min when running in release)
    let content = if cfg!(debug_assertions) {
        file.contents().to_string()
    } else {
        file.min_contents()
    };

    // returns the contents of the JS file
    Ok(HttpResponse::Ok()
        .content_type("text/javascript; charset=utf-8")
        .body(content))
}
"#;
