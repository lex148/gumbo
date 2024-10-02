use crate::change::Change;
use crate::errors::Result;

pub(crate) fn write_template() -> Result<Vec<Change>> {
    Ok(vec![
        Change::new("./src/controllers/greetings_controller.rs", CODE)?.add_parent_mod(),
        Change::new("./src/views/greetings/mod.rs", "")?.add_parent_mod(),
        Change::new("./src/views/greetings/index.rs", VIEW)?.add_parent_mod(),
        Change::new("./src/assets/js/greetings.js", JS)?,
    ])
}

static JS: &str = r#"
import { Controller } from "https://unpkg.com/@hotwired/stimulus/dist/stimulus.js"

class GreetingController extends Controller {

	greet() {
		alert("Hello !!!")
	}

}

Stimulus.register("greeting", GreetingController)
"#;

static CODE: &str = r#"
use gumbo_lib::view::render;
use gumbo_lib::session::Session;
use crate::errors::Result;
use actix_web::{get, web::Path, HttpResponse};
use welds::prelude::*;

#[get("/")]
async fn index(session: Option<Session>) -> Result<HttpResponse> {
    use crate::views::greetings::index::{View, ViewArgs};
    let args = ViewArgs::new(session);
    render::<View,_, _>(args).await
}

#[get("/restricted")]
async fn index_restricted(session: Session) -> Result<HttpResponse> {
    use crate::views::greetings::index::{View, ViewArgs};
    let args = ViewArgs::new(Some(session));
    render::<View,_, _>(args).await
}
"#;

static VIEW: &str = r#"
use gumbo_lib::session::Session;
use gumbo_lib::javascript::js_path;
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

        <div class="w-fill m-auto" data-controller="greeting">

          <img data-action="click->greeting#greet" src="/assets/gumbo.webp" class="h-80 block m-auto"/>

          <h1 class="text-4xl">{ "Welcome to Gumbo" }</h1>

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


          <div class="max-w-lg mt-9">
            <div class="text-2xl pt-3 pb-3">{"Faster Development"}
              <span class="text-red-600 pl-2">{" !important"}</span>
            </div>
            <div class="text-gray-600 text-sm" >{"There are a couple of things you should do that make a huge differents while developing"}</div>
            <ol class="text-gray-600 text-sm list-decimal ml-4 mt-1" >
                <li class="list-item" >
                  <a href="https://github.com/rui314/mold#user-content-how-to-use">{"Setup mold as your linker"}</a>
                </li>
                <li class="list-item" >
                  {"If you use cargo watch to rebuild while doing a task like styling or working with your HTML make SURE you disable the delay"}
                </li>
            </ol>

            <div class="inline-block p-1 m-1 pt-2 bg-gray-200 text-gray-500 border border-gray-300 rounded-md ">
              {"cargo watch -d 0.0 -x run"}
            </div>

          </div>


        </div>
        <script type="module" src={ js_path("greetings").unwrap() } />

      </MainLayout>
    }
}
"#;
