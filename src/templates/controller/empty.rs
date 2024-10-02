use crate::action::Action;
use crate::names::Names;

pub(crate) fn template(names: &Names, action: &Action) -> String {
    if &action.method == "get" {
        return template_with_view(names, action);
    }
    template_without_view(names, action)
}

pub fn template_with_view(names: &Names, action: &Action) -> String {
    let indexaction = &names.view_mod;
    let viewmod = &names.view_mod;
    let rawname = &action.name;
    let name = action.fn_name();
    let method = &action.method;

    format!(
        r#"
#[{method}("/{indexaction}/{rawname}")]
pub(crate) async fn {name}() -> Result<HttpResponse> {{
    use crate::views::{viewmod}::{name}::{{View, ViewArgs}};
    let args = ViewArgs::new();
    render::<View,_ , _>(args).await
}}
"#
    )
}

pub fn template_without_view(names: &Names, action: &Action) -> String {
    let indexaction = &names.view_mod;
    let rawname = &action.name;
    let name = action.fn_name();
    let method = &action.method;

    format!(
        r#"
#[{method}("/{indexaction}/{rawname}")]
pub(crate) async fn {name}() -> Result<HttpResponse> {{
    todo!();
}}
"#
    )
}
