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
    fields: &[Field],
) -> Result<(), TemplateError> {
    // the this module
    let view_mod = format!("./src/views/{}/mod.rs", &names.view_mod);
    append_module(root_path, &view_mod, "form")?;

    let code = build_crud_template(names, fields);

    let action_path = format!("./src/views/{}/form.rs", &names.view_mod);
    let mut path = root_path.to_path_buf();
    path.push(action_path);
    ensure_directory_exists(&path)?;

    let mut file = File::create(path)?;
    file.write_all(code.trim().as_bytes())?;

    Ok(())
}

fn build_crud_template(names: &Names, fields: &[Field]) -> String {
    let modelmod = &names.model_mod;
    let modelstruct = &names.model_struct;
    let fieldscode: Vec<_> = fields.iter().map(|f| build_field_code(names, f)).collect();
    let fieldscode = fieldscode.join("\n");

    format!(
        r#"
use crate::models::{modelmod}::{modelstruct};
use std::sync::Arc;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub(crate) struct ViewArgs {{
    #[prop_or_default]
    pub(crate) {modelmod}: Option<Arc<{modelstruct}>>,
    pub(crate) action: AttrValue,
    pub(crate) method: AttrValue,
}}

#[function_component]
pub(crate) fn Form(args: &ViewArgs) -> Html {{
    html! {{
        <form action={{args.action.clone()}} method={{args.method.clone()}} >
{fieldscode}
            <input type="submit" value={{"Save"}} />
        </form>
    }}
}}
"#
    )
}

fn build_field_code(names: &Names, field: &Field) -> String {
    let fieldname = &field.name;
    let modelmod = &names.model_mod;

    let fieldread = match field.ty {
        Type::String => format!(
            "args.{modelmod}.as_ref().map(|x| x.{fieldname}.to_string()).unwrap_or_default()"
        ),
        _ => format!(
            "args.{modelmod}.as_ref().map(|x| x.{fieldname}.to_string()).unwrap_or_default()"
        ),
    };

    format!(
        r#"
            <div>
                <label for={{"{modelmod}_{fieldname}"}} >{{"{fieldname}"}}</label>
                <input type={{"text"}} id={{"{modelmod}_{fieldname}"}} name={{"{modelmod}[{fieldname}]"}} value={{ {fieldread} }} />
            </div>
    "#
    )
}
