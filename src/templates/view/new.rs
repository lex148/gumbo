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
use gumbo_lib::Session;
use super::form::Form;
use std::sync::Arc;
use yew::prelude::*;

#[derive(Properties, PartialEq, Default)]
pub(crate) struct ViewArgs {{
    #[prop_or_default]
    pub(crate) session: Option<Arc<Session>>,
    #[prop_or_default]
    pub(crate) {modelmod}: Option<Arc<{modelstruct}>>,
}}
impl ViewArgs {{
    pub(crate) fn new(session: Option<Session>, {modelmod}: Option<Arc<{modelstruct}>>) -> Self {{
        Self {{ 
          session: session.map( Arc::new ),
          {modelmod} 
        }}
    }}
}}

#[function_component]
pub(crate) fn New(args: &ViewArgs) -> Html {{
    html! {{
        <>
          <MainLayout session={{ args.session.clone() }}>
            <Form action={{ "/{action}" }} method={{"POST"}} {modelmod}={{ args.{modelmod}.clone() }} />
          </MainLayout>
        </>
    }}
}}
"#
    )
}
