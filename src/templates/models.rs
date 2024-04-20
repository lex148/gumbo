use super::modrs::append_module;
use super::{ensure_directory_exists, TemplateError};
use crate::fields::Field;
use crate::names::Names;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::str::FromStr;

pub(crate) fn write_template(
    root_path: &Path,
    names: &Names,
    fields: &[Field],
) -> Result<(), TemplateError> {
    let mut path = root_path.to_path_buf();
    path.push(&names.model_path);
    ensure_directory_exists(&path)?;

    let mut file = File::options()
        .write(true)
        .truncate(true)
        .create(true)
        .open(&path)?;
    let code = build(names, fields)?;
    file.write_all(code.as_bytes())?;

    append_module(root_path, "./src/models/mod.rs", &names.model_mod)?;
    Ok(())
}

fn build(names: &Names, fields: &[Field]) -> Result<String, TemplateError> {
    let id_field = [Field::from_str("id:int").unwrap()];
    let innerds: Vec<String> = id_field
        .iter()
        .chain(fields.iter())
        .map(field_line)
        .collect();
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

    Ok(format!("{head}{innerds}\n}}"))
}

fn field_line(field: &Field) -> String {
    let Field { name, ty, null } = field;
    let ty = ty.rust_type();
    if *null {
        format!("  {name}: Option<{ty}>,")
    } else {
        format!("  {name}: {ty},")
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
  id: i32,
  item: Option<String>,
  price: f32,
}"#
            .trim()
        )
    }
}
