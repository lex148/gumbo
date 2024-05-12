use crate::fields::{Field, Type};
use crate::names::Names;
use crate::templates::ensure_directory_exists;
use crate::templates::modrs::append_module;
use crate::templates::TemplateError;
use std::fs::File;
use std::io::Write;
use std::path::Path;

/// Writes all the actions views
pub(crate) fn write_crud_template(
    root_path: &Path,
    names: &Names,
    _fields: &[Field],
) -> Result<(), TemplateError> {
    // the this module
    let view_mod = format!("./src/views/{}/mod.rs", &names.view_mod);
    append_module(root_path, &view_mod, "new")?;

    let code = build_crud_template(names);

    let action_path = format!("./src/views/{}/new.rs", &names.view_mod);
    let mut path = root_path.to_path_buf();
    path.push(action_path);
    ensure_directory_exists(&path)?;

    let mut file = File::create(path)?;
    file.write_all(code.trim().as_bytes())?;

    Ok(())
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
