use crate::change::Change;
use crate::errors::Result;
use crate::fields::Field;
use crate::names::Names;
use std::str::FromStr;

pub(crate) fn write_template(names: &Names, fields: &[Field]) -> Result<Vec<Change>> {
    let path = &names.model_path;
    let code = build(names, fields)?;
    Ok(vec![Change::new_from_path(path, code)?.add_parent_mod()])
}

fn build(names: &Names, fields: &[Field]) -> Result<String> {
    // the ID field, or use a default
    let id_field: Option<Field> = fields.iter().find(|x| x.name == "id").cloned();
    let id_field: Field = id_field.unwrap_or(Field::from_str("id:uuid")?);
    let id_field = [id_field];

    // the fields without any IDs
    let fields = fields.iter().filter(|x| x.name != "id");

    let innerds: Vec<String> = id_field.iter().chain(fields).map(field_line).collect();
    let innerds: String = innerds.join("\n");
    let table_name = &names.table_name;
    let model_name = &names.model_struct;

    let head = format!(
        r#"use welds::prelude::*;

#[derive(Debug, WeldsModel, PartialEq)]
#[welds(table = "{table_name}")]
pub(crate) struct {model_name} {{
"#
    );

    Ok(format!("{head}  #[welds(primary_key)]\n{innerds}\n}}"))
}

fn field_line(field: &Field) -> String {
    let Field { name, ty, null } = field;
    let ty = ty.rust_type();
    if *null {
        format!("  pub {name}: Option<{ty}>,")
    } else {
        format!("  pub {name}: {ty},")
    }
}

#[cfg(test)]
mod tests {

    use std::str::FromStr;

    use super::*;

    #[test]
    fn create_example_model() {
        let n = Names::new("InventoryLevels");
        let fields = vec![
            Field::from_str("item:string:null").unwrap(),
            Field::from_str("price:float").unwrap(),
        ];

        let code = build(&n, &fields).unwrap();

        assert_eq!(
            code,
            r#"
use welds::prelude::*;

#[derive(Debug, WeldsModel, PartialEq)]
#[welds(table = "inventory_levels")]
pub(crate) struct InventoryLevel {
  pub id: uuid::Uuid,
  pub item: Option<String>,
  pub price: f32,
}"#
            .trim()
        )
    }
}
