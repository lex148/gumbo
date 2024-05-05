use crate::action::Action;
use crate::names::Names;
use crate::templates::ensure_directory_exists;
use crate::templates::modrs::append_module;
use crate::templates::TemplateError;
use std::fs::File;
use std::io::Write;
use std::path::Path;

/// Writes all the actions views
pub(crate) fn write_template(
    root_path: &Path,
    names: &Names,
    action: &Action,
) -> Result<(), TemplateError> {
    let actionname = action.fn_name();
    let view_mod_name = &names.view_mod;
    let viewmod_path = format!("./src/views/{view_mod_name}/mod.rs");
    append_module(root_path, &viewmod_path, &actionname)?;

    let code = build_template(view_mod_name, &actionname);

    let action_path = format!("./src/views/{view_mod_name}/{actionname}.rs");
    let mut path = root_path.to_path_buf();
    path.push(action_path);
    ensure_directory_exists(&path)?;

    let mut file = File::create(path)?;
    file.write_all(code.trim().as_bytes())?;

    Ok(())
}

fn build_template(viewmod: &str, action: &str) -> String {
    format!(
        r#"
use std::sync::Arc;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub(crate) struct ViewArgs {{ }}

impl ViewArgs {{
    pub(crate) fn new() -> Self {{
        Self {{  }}
    }}
}}

#[function_component]
pub(crate) fn View(args: &ViewArgs) -> Html {{
    html! {{
        <>
            <span>{{"view: views/{viewmod}/{action}"}}</span>
        </>
    }}
}}
"#
    )
}
