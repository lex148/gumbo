use super::usings;
use crate::fields::{Field, Type};
use crate::names::Names;
use crate::templates::ensure_directory_exists;
use crate::templates::modrs::append_module;
use crate::templates::TemplateError;
use codegen::{Scope, Struct};
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
    append_module(root_path, &view_mod, "single")?;

    let code = build_crud_template(names, fields);

    let action_path = format!("./src/views/{}/single.rs", &names.view_mod);
    let mut path = root_path.to_path_buf();
    path.push(action_path);
    ensure_directory_exists(&path)?;

    let mut file = File::options()
        .write(true)
        .truncate(true)
        .create(true)
        .open(path)?;
    file.write_all(code.as_bytes())?;

    Ok(())
}

fn view_args(names: &Names) -> Struct {
    let mut st = Struct::new("ViewArgs");
    st.vis("pub(crate)");
    st.attr("derive(Properties, PartialEq)");
    st.new_field(&names.model_mod, format!("Arc<{}>", &names.model_struct))
        .vis("pub(crate)");
    st
}

fn view(names: &Names, fields: &[Field]) -> String {
    let name = &names.model_struct;
    let modname = &names.model_mod;

    let parts: Vec<_> = fields.iter().map(|f| build_field_code(names, f)).collect();
    let fieldscode = parts.join("\n");

    format!(
        r#"
#[function_component]
pub(crate) fn {name}View(args: &ViewArgs) -> Html {{
    html! {{
        <>
            {fieldscode}
        </>
    }}
}}
"#
    )
}

/// Writes all the actions views
pub(crate) fn build_crud_template(names: &Names, fields: &[Field]) -> String {
    let mut s = Scope::new();
    s.push_struct(view_args(names));
    format!(
        "{}\n{}\n{}",
        usings(names),
        s.to_string(),
        view(names, fields)
    )
}

fn build_field_code(names: &Names, field: &Field) -> String {
    let fieldname = &field.name;
    let modelmod = &names.model_mod;

    let fieldread = match field.ty {
        Type::String => format!("args.{modelmod}.{fieldname}.to_string()"),
        _ => format!("args.{modelmod}.{fieldname}.to_string()"),
    };

    format!(
        r#"
            <div>
                <label>{{"{fieldname}"}}</label>
                <span>{{ {fieldread} }}</span>
            </div>
    "#
    )
}
