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
use crate::helpers::render;
use crate::{errors::Result, models::session::Session};
use actix_web::{get, web::Path, HttpResponse};
use welds::prelude::*;

#[get("/")]
async fn index(session: Option<Session>) -> Result<HttpResponse> {
    use crate::views::greetings::index::{View, ViewArgs};
    let args = ViewArgs::new(session);
    render::<View, _>(args).await
}

#[get("/restricted")]
async fn index_restricted(session: Session) -> Result<HttpResponse> {
    use crate::views::greetings::index::{View, ViewArgs};
    let args = ViewArgs::new(Some(session));
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
use crate::models::session::Session;
use crate::views::layouts::MainLayout;
use std::sync::Arc;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ViewArgs {
    session: Option<Arc<Session>>,
}

impl ViewArgs {
    pub fn new(session: Option<Session>) -> ViewArgs {
        ViewArgs {
            session: session.map(Arc::new),
        }
    }
}

#[function_component]
pub(crate) fn View(args: &ViewArgs) -> Html {
    html! {
      <MainLayout>
        <h1>{ format!("Hello, World") }</h1>

        if let Some(session) = args.session.clone() {
           <br/>
           <h1>{ format!("You are logged in as: {}", session.sub()) }</h1>
           <br/>
           <a href="/auth/logout" data-turbo-method="delete" data-turbo-confirm="Are you sure?" rel="nofollow">{"Logout"}</a>
        } else {
           <br/>
           <a href="/auth/login/google" data-turbo="false">{"Login With Google"}</a>
        }

      </MainLayout>
    }
}
"#;
