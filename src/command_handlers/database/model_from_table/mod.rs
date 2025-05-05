use crate::change::Change;
use crate::errors::{GumboError::TableNotFound, Result};
use crate::fields::Type;
use crate::fields::{Field, Visibility};
use crate::names::Names;
use crate::templates::models::{build as build_full, build_struct};
use welds::Client;
use welds::detect::TableDef;
use welds::detect::find_all_tables;
use welds::model_traits::TableIdent;
use welds::writers::types::recommended_rust_type;

mod code;
use code::{find_model, read_fields};

// update or generate a model from an existing table in the database
pub(crate) async fn run(table_name: &str) -> Result<Vec<Change>> {
    let names = Names::new(table_name);

    let mut parts: Vec<_> = table_name.trim().split(".").collect();
    let table_name: &str = parts.pop().unwrap();

    // get the columns/fields from the database.
    let pool = welds::connections::connect_from_env().await?;
    let table_def = get_table_def(&pool, table_name, &names).await?;
    let def_fields = into_fields(&table_def, pool.syntax());

    // get the columns, fields from the code model
    let path = &names.model_path;
    let code = std::fs::read_to_string(path).unwrap_or_default();
    let model_start_end: Option<(usize, usize)> = find_model(&code);
    let code_fields = match model_start_end {
        Some(start_end) => read_fields(&code[start_end.0..start_end.1]),
        None => Vec::default(),
    };
    let model_start_end = model_start_end.unwrap_or_default();

    // build the new list of column for the struct
    let updated_fields = update_column_list(&code_fields, &def_fields);

    let struct_code = match code.is_empty() {
        true => build_full(&names, &updated_fields)?,
        false => build_struct(&names, &updated_fields)?,
    };

    //write the new struct
    let new_code = replace_range(&code, model_start_end.0, model_start_end.1, &struct_code);

    Ok(vec![
        Change::new_from_path(path, new_code)?.add_parent_mod(),
    ])
}

fn replace_range(old: &str, start: usize, end: usize, new_content: &str) -> String {
    let mut s = old.to_string();
    s.replace_range(start..end, new_content);
    s
}

/// Read a tabledef into Fields to be used to update the model
fn into_fields(def: &TableDef, syntax: welds::Syntax) -> Vec<Field> {
    def.columns()
        .iter()
        .map(|c| {
            let sql_type = c.ty().to_string();

            let rust_type = recommended_rust_type(syntax, &sql_type)
                .map(|p| p.full_rust_type())
                .unwrap_or_else(|| sql_type.clone());

            Field {
                visibility: Visibility::default(),
                primary_key: c.primary_key(),
                welds_ignored: false,
                name: c.name().to_string(),
                ty: Type::Raw(sql_type, rust_type),
                null: c.null() && !c.primary_key(),
            }
        })
        .collect()
}

/// Updates the list of columns to be
/// the new list of columns that is expected in the updated model
fn update_column_list<'f>(code_fields: &'f [Field], def: &'f [Field]) -> Vec<Field> {
    // split out the id fields from the normal onces.
    let (def_id_fields, def_fields): (Vec<_>, Vec<_>) = def.iter().partition(|f| f.primary_key);
    let (_code_id_fields, code_fields): (Vec<_>, Vec<_>) =
        code_fields.iter().partition(|f| f.primary_key);

    // make our copy of the fields to work with
    let mut fields: Vec<Field> = def_fields.iter().copied().cloned().collect();

    // keep the visibility from the source codes visibility
    for field in &mut fields {
        if let Some(found) = code_fields.iter().find(|f| f.name == field.name) {
            field.visibility = found.visibility;
        }
    }

    // add any ignored fields from the code.
    for ignored in code_fields.iter().copied().filter(|f| f.welds_ignored) {
        fields.push(ignored.clone());
    }

    // default to the id types the user has defined, they don't updated
    let id_fields = def_id_fields;

    id_fields
        .iter()
        .copied()
        .chain(fields.iter())
        .cloned()
        .collect()
}

async fn get_table_def(client: &dyn Client, table_name: &str, names: &Names) -> Result<TableDef> {
    let default_schema = TableIdent::default_namespace(client.syntax()).map(|x| x.to_string());
    let schema_name = names.schema_name.as_ref().or(default_schema.as_ref());
    let ident = TableIdent::new(table_name.to_string(), schema_name);

    let tables = find_all_tables(client).await?;

    let found = tables
        .iter()
        .find(|t| t.ident().eq(&ident))
        .ok_or_else(|| TableNotFound(table_name.to_string()))?;

    Ok(found.clone())
}

#[cfg(test)]
mod tests;
