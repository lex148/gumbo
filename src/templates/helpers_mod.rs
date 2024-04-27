use super::{ensure_directory_exists, TemplateError};
use std::fs::File;
use std::io::Write;
use std::path::Path;

pub(crate) fn write_template(root_path: &Path) -> Result<(), TemplateError> {
    let mut path = root_path.to_path_buf();
    path.push(Path::new("./src/helpers/mod.rs"));
    ensure_directory_exists(&path)?;
    let mut file = File::options()
        .write(true)
        .truncate(true)
        .create(true)
        .open(&path)?;
    file.write_all(CODE.as_bytes())?;
    Ok(())
}

static CODE: &str = r#"
use crate::errors::Result;
use actix_web::HttpResponse;
use yew::html::BaseComponent;
use yew::ServerRenderer;

/// Render a Yew view to send out in an Actix Response
pub(crate) async fn render<V, VM>(args: VM) -> Result<HttpResponse>
where
    V: BaseComponent,
    V: BaseComponent<Properties = VM>,
    VM: Send + 'static,
{
    let renderer = ServerRenderer::<V>::with_props(|| args);
    let html = renderer.render().await;
    // add the doctype markup. Yew doesn't like to render this.
    let html = format!("<!DOCTYPE html>\n{html}");
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html))
}

/// Render a Yew view to send out in an Actix Response
/// Used when a form is not valid
pub(crate) fn redirect(path: impl Into<String>) -> Result<HttpResponse> {
    let path: String = path.into();
    Ok(HttpResponse::SeeOther()
        .insert_header(("Location", path))
        .finish())
}
"#;
