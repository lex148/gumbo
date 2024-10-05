use crate::names::Names;
use codegen::Scope;

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
        .arg("session", "Option<Session>")
        .arg("db", "DbClient")
        .attr(&attr);

    func.line(format!(
        "use crate::views::{name}::index::{{View, ViewArgs}};"
    ));
    func.line("let db = &*db.into_inner();");
    func.line(format!("let list = {st}::all().run(db).await?;"));
    func.line("let args = ViewArgs::new(session, list);");
    func.line("render::<View,_,_>(args).await");

    s.to_string()
}
