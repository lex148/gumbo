use codegen::{Impl, Scope, Struct};

use super::usings;
use crate::names::Names;
use crate::templates::ensure_directory_exists;
use crate::templates::modrs::append_module;
use crate::templates::TemplateError;
use std::fs::File;
use std::io::Write;
use std::path::Path;

/// Writes all the actions views
pub(crate) fn write_crud_template(root_path: &Path, names: &Names) -> Result<(), TemplateError> {
    // the this module
    let view_mod = format!("./src/views/{}/mod.rs", &names.view_mod);
    append_module(root_path, &view_mod, "index")?;

    let code = build_crud_template(names);

    let action_path = format!("./src/views/{}/index.rs", &names.view_mod);
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
    st.new_field("list", format!("Vec<Arc<{}>>", &names.model_struct))
        .vis("pub(crate)");
    st
}
fn impl_view_args(names: &Names) -> Impl {
    let mut imp = Impl::new("ViewArgs");
    let new = imp.new_fn("new");
    new.vis("pub(crate)");
    let ty = format!("Vec<welds::state::DbState<{}>>", &names.model_struct);
    new.arg("mut list", ty);
    new.ret("Self");
    new.line("Self { list: list.drain(..).map(|x| x.into_vm()).collect() }");
    imp
}

fn view(names: &Names) -> String {
    let name = &names.model_struct;
    let single = &names.model_mod;
    let title = &names.title;
    format!(
        r#"
#[function_component]
pub(crate) fn View(args: &ViewArgs) -> Html {{
    html! {{
        <MainLayout>
          <h1>{{"List of {title}"}}</h1>
          {{ args.list.iter().map(|x| html!{{ <{name}View {single}={{x}} />}}  ).collect::<Html>() }}

          <br/>
          <a href={{"/cars/new"}} >{{"New {title}"}}</a>
        </MainLayout>
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
