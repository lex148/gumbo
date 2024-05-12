use super::modrs::append_module;
use super::{ensure_directory_exists, TemplateError};
use std::fs::File;
use std::io::Write;
use std::path::Path;

pub(crate) fn write_template(root_path: &Path) -> Result<(), TemplateError> {
    let mut path = root_path.to_path_buf();
    path.push(Path::new("./src/views/layouts/mod.rs"));
    ensure_directory_exists(&path)?;
    let mut file = File::options()
        .write(true)
        .truncate(true)
        .create(true)
        .open(&path)?;
    file.write_all(CODE_LAYOUTS.as_bytes())?;
    write_layout_main(root_path)?;
    append_module(root_path, "./src/views/mod.rs", "layouts")?;
    Ok(())
}

pub(crate) fn write_layout_main(root_path: &Path) -> Result<(), TemplateError> {
    let mut path = root_path.to_path_buf();
    path.push(Path::new("./src/views/layouts/main.rs"));
    ensure_directory_exists(&path)?;
    let mut file = File::options()
        .write(true)
        .truncate(true)
        .create(true)
        .open(&path)?;
    file.write_all(CODE_LAYOUT_MAIN.as_bytes())?;
    Ok(())
}

static CODE_LAYOUTS: &str = r#"
mod main;
pub(crate) use main::Layout as MainLayout;
"#;

static CODE_LAYOUT_MAIN: &str = r#"
use yew::prelude::*;
use yew::{html, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct LayoutProps {
    #[prop_or_default]
    pub children: Html,
}

#[function_component]
pub(crate) fn Layout(props: &LayoutProps) -> Html {
    let turbo = "https://cdn.jsdelivr.net/npm/@hotwired/turbo@8.0.4/dist/turbo.es2017-umd.js";

    html! {
      <html lang="en">
        <head>
          <meta charset="UTF-8" />
          <meta name="viewport" content="width=device-width, initial-scale=1.0" />
          <meta http-equiv="X-UA-Compatible" content="ie=edge" />
          <title>{"My Website"}</title>
          <link rel="stylesheet" href="/app.css" />
          <link rel="icon" href="./favicon.ico" type="image/x-icon" />
          <script defer=true src={turbo} />
        </head>
        <body class="bg-[#fff9de]">
          <main>
              { props.children.clone() }
          </main>
        </body>
      </html>
    }
}
"#;
