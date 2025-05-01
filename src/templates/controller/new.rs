use crate::names::Names;
use codegen::Scope;

pub(crate) fn crud_template(names: &Names) -> String {
    let mut s = Scope::new();
    let func = s.new_fn("new");
    let name = &names.view_mod;
    let route = format!("/{}/new", name);
    let attr = format!("get(\"{route}\")");
    func.set_async(true)
        .vis("pub(crate)")
        .ret("Result<HttpResponse>")
        .arg("session", "Option<Session>")
        .attr(&attr);

    let viewmod = &names.view_mod;
    func.line(format!(
        "use crate::views::{viewmod}::new::{{New, ViewArgs}};"
    ));
    func.line("let args = ViewArgs::new(session, None);");
    func.line("render::<New,_,_>(args).await");

    s.to_string()
}
