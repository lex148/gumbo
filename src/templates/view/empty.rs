use crate::action::Action;
use crate::change::Change;
use crate::errors::Result;
use crate::names::Names;

/// Writes all the actions views
pub(crate) fn write_template(names: &Names, action: &Action) -> Result<Change> {
    let actionname = action.fn_name();
    let view_mod_name = &names.view_mod;
    let code = build_template(view_mod_name, &actionname);
    let path = format!("./src/views/{view_mod_name}/{actionname}.rs");
    Ok(Change::new(path, code)?.add_parent_mod())
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
