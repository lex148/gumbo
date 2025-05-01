use crate::fields::Type;
use crate::{fields::Field, names::Names};
use welds::migrations::prelude::Type as MigrationType;
use welds::{Client, migrations::prelude::*};

async fn test_db() -> welds::connections::any::AnyClient {
    let connection_string = "sqlite::memory:";
    let client = welds::connections::connect(connection_string)
        .await
        .unwrap();
    up(&client, &[create_peoples_table]).await.unwrap();
    client
}

// A simple migration to setup the peoples table.
fn create_peoples_table(_: &TableState) -> welds::errors::Result<MigrationStep> {
    let m = create_table("people")
        .id(|c| c("id", MigrationType::Int))
        .column(|c| c("name", MigrationType::String).create_unique_index())
        .column(|c| c("age", MigrationType::IntBig))
        .column(|c| c("likes_apples", MigrationType::Bool).is_null());
    Ok(MigrationStep::new("create_peoples_table", m))
}

#[tokio::test]
async fn should_be_able_to_connect() {
    // no errors
    let _conn = test_db().await;
}

// const EXPECTED_HEADER: &str = "use welds::prelude::*;";
//
// const EXPECTED_MODEL: &str = "
// #[derive(Debug, WeldsModel, PartialEq)]
// pub(crate) struct person {
//     #[welds(primary_key)]
//     id: string,
//     age: i64,
//     likes_apples: Option<bool>
// }
// ";

#[tokio::test]
async fn should_be_able_to_setup_fields_for_basic() {
    use std::str::FromStr;

    let db = test_db().await;
    let table_name = "people";
    let names = Names::new(table_name);
    let def = super::get_table_def(&db, table_name, &names).await.unwrap();
    let def_fields = super::into_fields(&def, db.syntax());
    let fields = super::update_column_list(&[], &def_fields);

    let mut f = Field::from_str("id:int").unwrap();
    f.ty = Type::Raw("INTEGER".to_string(), "i32".to_string());
    assert_eq!(fields[0], f, "INDEX: 0");

    let mut f = Field::from_str("age:intbig").unwrap();
    f.ty = Type::Raw("INTEGER".to_string(), "i32".to_string());
    assert_eq!(fields[1], f, "INDEX: 1");

    let mut f = Field::from_str("likes_apples:bool:null").unwrap();
    f.ty = Type::Raw("BOOLEAN".to_string(), "bool".to_string());
    assert_eq!(fields[2], f, "INDEX: 2");

    let mut f = Field::from_str("name:string").unwrap();
    f.ty = Type::Raw("TEXT".to_string(), "String".to_string());
    assert_eq!(fields[3], f, "INDEX: 3");
}
