use crate::change::Change;
use crate::errors::Result;
use crate::fields::{Field, Type};
use crate::names::Names;

/// Writes all the actions views
pub(crate) fn write_crud_template(names: &Names, fields: &[Field]) -> Result<Change> {
    let path = format!("./src/views/{}/form.rs", &names.view_mod);
    let code = build_crud_template(names, fields);
    Ok(Change::new(path, code)?.add_parent_mod())
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
        <form action={{args.action.clone()}} method={{"POST"}} >
            <input type="hidden" name={{"_method"}} value={{args.method.clone()}} />
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
