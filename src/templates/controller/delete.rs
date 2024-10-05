use crate::names::Names;

pub(crate) fn crud_template(names: &Names) -> String {
    let indexaction = &names.view_mod;
    let model = &names.model_mod;
    let model_struct = &names.model_struct;
    format!(
        r#"
#[delete("/{indexaction}/{{id}}")]
pub(crate) async fn delete(db: DbClient, path: Path<i32>, _session: Option<Session>) -> Result<HttpResponse> {{
    let id = path.into_inner();
    let mut {model} = {model_struct}::find_by_id(db.as_ref(), id)
        .await?
        .ok_or(ServerError::ResourceNotFound)?;
    {model}.delete(db.as_ref()).await?;
    // redirect
    redirect("/{indexaction}")
}}
"#
    )
}
