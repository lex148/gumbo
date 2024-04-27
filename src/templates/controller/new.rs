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
        .attr(&attr);

    func.line("use crate::views::cars::new::{New, ViewArgs};");
    func.line("let args = ViewArgs::default();");
    func.line("render::<New,_>(args).await");

    s.to_string()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn should_be_able_to_write_action() {
        let names = Names::new("cars");
        let code = crud_template(&names);
        assert_eq!(code, EXPECTED_CRUD.trim())
    }

    static EXPECTED_CRUD: &str = r#"
#[get("/cars/new")]
pub(crate) async fn new() -> Result<HttpResponse> {
    use crate::views::cars::new::{New, ViewArgs};
    let args = ViewArgs::default();
    render::<New,_>(args).await
}
"#;
}
