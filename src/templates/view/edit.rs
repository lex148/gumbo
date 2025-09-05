use crate::change::Change;
use crate::errors::Result;
use crate::fields::Field;
use crate::names::Names;

pub(crate) fn write_crud_template(names: &Names, _fields: &[Field]) -> Result<Change> {
    let path = format!("./src/views/{}/edit.rs", &names.view_mod);
    let code = build_crud_template(names);
    Ok(Change::new(path, code)?.add_parent_mod())
}

fn build_crud_template(names: &Names) -> String {
    let modelmod = &names.model_mod;
    let modelstruct = &names.model_struct;
    let action = &names.view_mod;

    format!(
        r#"
use crate::models::{modelmod}::{modelstruct};
use crate::views::layouts::MainLayout;
use super::form::Form;
use gumbo_lib::Session;
use gumbo_lib::view::app_path;
use std::sync::Arc;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub(crate) struct ViewArgs {{
    #[prop_or_default]
    pub(crate) session: Option<Arc<Session>>,
    pub(crate) {modelmod}: Arc<{modelstruct}>,
}}
impl ViewArgs {{
    pub(crate) fn new(session: Option<Session>, {modelmod}: Arc<{modelstruct}>) -> Self {{
        Self {{ 
          session: session.map( Arc::new ),
          {modelmod} 
        }}
    }}
}}

#[function_component]
pub(crate) fn Edit(args: &ViewArgs) -> Html {{
    let route = app_path(format!("/{action}/{{}}", args.{modelmod}.id ));
    html! {{
        <>
          <MainLayout session={{ args.session.clone() }}>
            <Form action={{ route }} method={{"PATCH"}} {modelmod}={{ args.{modelmod}.clone() }} />
          </MainLayout>
        </>
    }}
}}
"#
    )
}
