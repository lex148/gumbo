use super::ensure_directory_exists;
use super::modrs::append_module;
use super::TemplateError;
use std::fs::File;
use std::io::Write;
use std::path::Path;

pub(crate) fn write_template(root_path: &Path) -> Result<(), TemplateError> {
    let mut path = root_path.to_path_buf();
    path.push("./src/controllers/greetings_controller.rs");
    ensure_directory_exists(&path)?;
    let mut file = File::options()
        .write(true)
        .truncate(true)
        .create(true)
        .open(&path)?;

    file.write_all(CODE.as_bytes())?;

    append_module(
        root_path,
        "./src/controllers/mod.rs",
        "greetings_controller",
    )?;

    append_module(root_path, "./src/views/mod.rs", "greetings")?;
    append_module(root_path, "./src/views/greetings/mod.rs", "index")?;

    write_view_index(root_path)?;

    Ok(())
}

static CODE: &str = r#"
use crate::errors::Result;
use crate::helpers::render;
use actix_web::{get, web::Path, HttpResponse};
use welds::prelude::*;

#[get("/")]
async fn index() -> Result<HttpResponse> {
    use crate::views::greetings::index::{View, ViewArgs};
    let args = ViewArgs { };
    render::<View, _>(args).await
}
"#;

pub(crate) fn write_view_index(root_path: &Path) -> Result<(), TemplateError> {
    let mut path = root_path.to_path_buf();
    path.push("./src/views/greetings/index.rs");
    ensure_directory_exists(&path)?;
    let mut file = File::options()
        .write(true)
        .truncate(true)
        .create(true)
        .open(&path)?;
    file.write_all(VIEW_CODE.as_bytes())?;
    Ok(())
}

static VIEW_CODE: &str = r#"
use crate::views::layouts::MainLayout;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ViewArgs { }

#[function_component]
pub(crate) fn View(args: &ViewArgs) -> Html {
    html! {
        <MainLayout>
          <h1>{ format!("Hello, World") }</h1>
        </MainLayout>
    }
}
"#;
