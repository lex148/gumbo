use crate::names::Names;

pub(crate) fn crud_template(names: &Names) -> String {
    let indexaction = &names.view_mod;
    let model = &names.model_mod;
    let model_struct = &names.model_struct;
    let viewmod = &names.view_mod;
    format!(
        r#"
#[get("/{indexaction}/{{id}}/edit")]
pub(crate) async fn edit(db: DbClient, path: Path<Uuid>, session: Option<Session>) -> Result<HttpResponse> {{
    let id = path.into_inner();
    let {model} = {model_struct}::find_by_id(db.as_ref(), id)
        .await?
        .ok_or(ServerError::ResourceNotFound)?
        .into_vm();

    use crate::views::{viewmod}::edit::{{Edit, ViewArgs}};
    let args = ViewArgs::new(session, {model});
    render::<Edit,_ , _>(args).await
}}
"#
    )
}
