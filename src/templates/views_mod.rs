use crate::change::Change;
use crate::errors::Result;

pub(crate) fn write_template(sitename: &str) -> Result<Vec<Change>> {
    let icon = include_bytes!("./favicon.ico");

    Ok(vec![
        Change::new("./src/views/layouts/mod.rs", CODE_LAYOUTS)?.add_parent_mod(),
        Change::new("./src/views/layouts/main.rs", code(sitename))?,
        Change::new("./src/assets/favicon.ico", icon.as_slice())?,
    ])
}

static CODE_LAYOUTS: &str = r#"
mod main;
pub(crate) use main::Layout as MainLayout;
"#;

fn code(sitename: &str) -> String {
    format!(
        r#"
use yew::prelude::*;
use yew::{{html, Html, Properties}};

#[derive(Properties, PartialEq)]
pub struct LayoutProps {{
    #[prop_or_default]
    pub children: Html,
}}

#[function_component]
pub(crate) fn Layout(props: &LayoutProps) -> Html {{
    let turbo = "https://cdn.jsdelivr.net/npm/@hotwired/turbo@8.0.4/dist/turbo.es2017-umd.js";

    html! {{
      <html lang="en">
        <head>
          <meta charset="UTF-8" />
          <meta name="viewport" content="width=device-width, initial-scale=1.0" />
          <meta http-equiv="X-UA-Compatible" content="ie=edge" />
          <title>{{"{sitename}"}}</title>
          <link rel="stylesheet" href="/app.css" />
          <link rel="icon" href="/assets/favicon.ico" type="image/x-icon" />
          <script defer=true src={{turbo}} />
        </head>
        <body class="bg-[#fff9de]">
          <main class="container mx-auto mt-12 mb-12 flex">
              {{ props.children.clone() }}
          </main>
        </body>
      </html>
    }}
}}
"#
    )
}
