use super::usings;
use crate::change::Change;
use crate::errors::Result;
use crate::fields::{Field, Type};
use crate::names::Names;
use codegen::{Scope, Struct};

/// Writes all the actions views
pub(crate) fn write_crud_template(names: &Names, fields: &[Field]) -> Result<Change> {
    let path = format!("./src/views/{}/single.rs", &names.view_mod);
    let code = build_crud_template(names, fields);
    Ok(Change::new(path, code)?.add_parent_mod())
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
                <span class="ml-3 font-bold">{{ {fieldread} }}</span>
            </div>
    "#
    )
}
