use codegen::{Impl, Scope, Struct};

use super::usings;
use crate::change::Change;
use crate::errors::Result;
use crate::names::Names;

/// Writes all the actions views
pub(crate) fn write_crud_template(names: &Names) -> Result<Change> {
    let path = format!("./src/views/{}/index.rs", &names.view_mod);
    let code = build_crud_template(names);
    Ok(Change::new(path, code)?.add_parent_mod())
}

fn view_args(names: &Names) -> Struct {
    let mut st = Struct::new("ViewArgs");
    st.vis("pub(crate)");
    st.attr("derive(Properties, PartialEq)");
    st.new_field("session", "Option<Arc<Session>>");
    st.new_field("list", format!("Vec<Arc<{}>>", &names.model_struct))
        .vis("pub(crate)");
    st
}
fn impl_view_args(names: &Names) -> Impl {
    let mut imp = Impl::new("ViewArgs");
    let new = imp.new_fn("new");
    new.vis("pub(crate)");
    new.arg("session", "Option<Session>");
    let ty = format!("Vec<welds::state::DbState<{}>>", &names.model_struct);
    new.arg("mut list", ty);
    new.ret("Self");
    new.line("Self { session: session.map(Arc::new), list: list.drain(..).map(|x| x.into_vm()).collect() }");
    imp
}

fn view(names: &Names) -> String {
    let name = &names.model_struct;
    let single = &names.model_mod;
    let viewmod = &names.view_mod;
    let title = &names.title;
    format!(
        r#"
#[function_component]
pub(crate) fn View(args: &ViewArgs) -> Html {{
    html! {{
        <MainLayout session={{ args.session.clone() }}>
          <h1>{{"List of {title}"}}</h1>
          {{ args.list.iter().map(|x| html!{{ <Row {single}={{x}} />}}  ).collect::<Html>() }}
          <br/>
          <a href={{"/{viewmod}/new"}} >{{"New {title}"}}</a>
        </MainLayout>
    }}
}}

#[derive(Properties, PartialEq)]
pub(crate) struct RowArgs {{
    pub(crate) {single}: Arc<{name}>,
}}

#[function_component]
fn Row(args: &RowArgs) -> Html {{
    html! {{
        <>
            <{name}View {single}={{args.{single}.clone()}} />
            <a href={{format!("/{viewmod}/{{}}/edit", args.{single}.id)}} >{{"Edit"}}</a>
            <br/>
        </>
    }}
}}

"#
    )
}

/// Writes all the actions views
pub(crate) fn build_crud_template(names: &Names) -> String {
    let mut s = Scope::new();

    s.push_struct(view_args(names));
    s.push_impl(impl_view_args(names));

    let single = format!("use super::single::{}View;\n", &names.model_struct);
    format!(
        "{}{}\n{}\n{}",
        usings(names),
        single,
        s.to_string(),
        view(names)
    )
}
