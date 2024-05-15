use crate::change::Change;
use crate::errors::Result;

pub(crate) fn write_template() -> Result<Vec<Change>> {
    Ok(vec![
        Change::new("./src/controllers/greetings_controller.rs", CODE)?.add_parent_mod(),
        Change::new("./src/views/greetings/mod.rs", "")?.add_parent_mod(),
        Change::new("./src/views/greetings/index.rs", VIEW)?.add_parent_mod(),
    ])
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

static VIEW: &str = r#"
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

        <div class="flex flex-column justify-center items-center">
          <img src="/assets/gumbo.webp" class="h-80 block "/>
        </div>
        <div class="flex flex-column justify-center items-center">
          <h1 class="text-4xl">{ "Welcome to Gumbo" }</h1>
        </div>

        <div class="flex flex-column justify-center items-center">
        <div class="max-w-lg">

          if let Some(session) = args.session.clone() {
            <br/>
            <h1>{ format!("You are logged in as: {}", session.sub()) }</h1>
            <br/>
            <a href="/auth/logout" data-turbo-method="delete" data-turbo-confirm="Are you sure?" rel="nofollow">{"Logout"}</a>
          } else {

            <br/>
            <a href="/auth/login/google" data-turbo="false">{"Login With Google"}</a>
            <br/>
            <div class="text-gray-600 text-sm">
              <div>{"Setup of google OAuth is very simple. Just add the client_id/client_secret to your .env or ENV VARs."}</div>
              <a href="https://www.youtube.com/watch?v=OKMgyF5ezFs" >{"Info about how to get an OAuth client_id/client_secret from google can be found here"}</a>
            </div>

            <br/>
            <a href="/auth/login/fakeoauth" data-turbo="false">{"Login With FakeOAuth"}</a>
            <br/>
            <div class="text-gray-600 text-sm">
              <div>{"FakeOAuth is a 'Fake' OAuth provider you can run locally while you build your app."}</div>
              <div>{"You can run FakeOAuth in docker like this"}</div>
              <div class="inline-block p-1 m-1 pt-2 bg-gray-200 text-gray-500 border border-gray-300 rounded-md ">{"docker run --rm -p \"127.0.0.1:5860:5860\" lex148/fakeoauth"}</div>
            </div>
          }

        </div>
        </div>

      </MainLayout>
    }
}
"#;
