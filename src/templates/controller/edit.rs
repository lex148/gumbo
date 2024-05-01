use crate::names::Names;

pub(crate) fn crud_template(names: &Names) -> String {
    let indexaction = &names.view_mod;
    let model = &names.model_mod;
    let model_struct = &names.model_struct;
    let viewmod = &names.view_mod;
    format!(
        r#"
#[get("/{indexaction}/{{id}}/edit")]
pub(crate) async fn edit(db: DbClient, path: Path<i32>) -> Result<HttpResponse> {{
    let id = path.into_inner();
    let {model} = {model_struct}::find_by_id(db.as_ref(), id)
        .await?
        .ok_or(ServerError::ResourceNotFound)?
        .into_vm();

    use crate::views::{viewmod}::edit::{{Edit, ViewArgs}};
    let args = ViewArgs::new({model});
    render::<Edit, _>(args).await
}}
"#
    )
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn should_be_able_to_write_action() {
        let names = Names::new("potato");
        let code = crud_template(&names);
        assert_eq!(code, EXPECTED_CRUD.trim())
    }

    static EXPECTED_CRUD: &str = r#"
#[get("/potatoes/1/edit")]
pub(crate) async fn edit(id: Path<i32>) -> Result<HttpResponse> {
    use crate::views::potatoes::new::{New, ViewArgs};
    let args = ViewArgs::default();
    render::<New,_>(args).await
}
"#;
}
