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
use std::sync::Arc;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub(crate) struct ViewArgs {{
    pub(crate) {modelmod}: Arc<{modelstruct}>,
}}
impl ViewArgs {{
    pub(crate) fn new({modelmod}: Arc<{modelstruct}>) -> Self {{
        Self {{ {modelmod} }}
    }}
}}

#[function_component]
pub(crate) fn Edit(args: &ViewArgs) -> Html {{
    let route = format!("/{action}/{{}}", args.{modelmod}.id );
    html! {{
        <>
          <MainLayout>
            <Form action={{ route }} method={{"PATCH"}} {modelmod}={{ args.{modelmod}.clone() }} />
          </MainLayout>
        </>
    }}
}}
"#
    )
}
