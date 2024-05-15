use crate::change::Change;
use crate::errors::Result;
use crate::fields::Field;
use crate::names::Names;

/// Writes all the actions views
pub(crate) fn write_crud_template(names: &Names, _fields: &[Field]) -> Result<Change> {
    let path = format!("./src/views/{}/new.rs", &names.view_mod);
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

#[derive(Properties, PartialEq, Default)]
pub(crate) struct ViewArgs {{
    #[prop_or_default]
    pub(crate) {modelmod}: Option<Arc<{modelstruct}>>,
}}

#[function_component]
pub(crate) fn New(args: &ViewArgs) -> Html {{
    html! {{
        <>
          <MainLayout>
            <Form action={{ "/{action}" }} method={{"POST"}} {modelmod}={{ args.{modelmod}.clone() }} />
          </MainLayout>
        </>
    }}
}}
"#
    )
}
