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
    append_module(root_path, &view_mod, "edit")?;

    let code = build_crud_template(names);

    let action_path = format!("./src/views/{}/edit.rs", &names.view_mod);
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
            <Form action={{ route }} method={{"PATCH"}} {modelmod}={{ args.{modelmod}.clone() }} />
        </>
    }}
}}
"#
    )
}
