use crate::names::Names;
use crate::templates::TemplateError;
use codegen::Block;
use codegen::Function;
use codegen::Scope;
use std::fs::File;
use std::io::Write;
use std::path::Path;

pub(crate) fn write_template(name: &str, controller_path: &Path) -> Result<(), TemplateError> {
    let mut file = File::options().append(true).open(controller_path)?;
    let code = empty_action(name);
    file.write_all(code.as_bytes())?;
    Ok(())
}

pub(crate) fn crud_template(names: &Names) -> String {
    let mut s = Scope::new();
    let func = s.new_fn("index");
    let name = &names.view_mod;
    let st = &names.model_struct;
    let route = format!("/{}", name);
    let attr = format!("get(\"{route}\")");
    func.set_async(true)
        .vis("pub(crate)")
        .ret("Result<HttpResponse>")
        .arg("db", "DbClient")
        .attr(&attr);

    func.line(format!(
        "use crate::views::{name}::index::{{View, ViewArgs}};"
    ));
    func.line("let db = &*db.into_inner();");
    func.line(format!("let list = {st}::all().run(db).await?;"));
    func.line("let args = ViewArgs::new(list);");
    func.line("render::<View,_>(args).await");

    s.to_string()
}

fn empty_action(name: &str) -> String {
    let mut s = Scope::new();
    let route = format!("/{name}");
    s.push_fn(get_action("index", &route, None));
    s.to_string()
}

fn get_action(action_name: &str, route: &str, body: Option<Vec<Block>>) -> Function {
    let attr = format!("get(\"{route}\")");
    let mut f = Function::new(action_name);
    f.set_async(true)
        .vis("pub(crate)")
        .ret("Result<HttpResponse>")
        .attr(&attr);
    match body {
        None => {
            f.line("todo!()");
        }
        Some(mut blox) => {
            blox.drain(..).for_each(|b| {
                f.push_block(b);
            });
        }
    };
    f
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn should_be_able_to_write_empty_action() {
        let code = empty_action("cars");
        assert_eq!(code, EXPECTED_EMPTY.trim())
    }

    static EXPECTED_EMPTY: &str = r#"
#[get("/cars")]
pub(crate) async fn index() -> Result<HttpResponse> {
    todo!()
}
"#;
}
