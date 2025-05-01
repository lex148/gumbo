use welds::detect::{DataType, TableDef, find_all_tables};
use welds::errors::Result;
use welds::model_traits::TableIdent;

pub(crate) async fn list_tables() -> Result<()> {
    let tables = fetch_db_tables().await?;
    for table in tables {
        if table.ty() == DataType::Table {
            let ident = table.ident();
            match ident.schema() {
                Some(schema) => println!("{}.{}", schema, ident.name()),
                None => println!("{}", ident.name()),
            }
        }
    }
    Ok(())
}

pub(crate) async fn fetch_db_tables() -> Result<Vec<TableDef>> {
    let pool = welds::connections::connect_from_env().await?;
    let mut tables = find_all_tables(&pool).await?;
    tables.sort_by(|a, b| a.ident().name().cmp(b.ident().name()));
    tables.sort_by(|a, b| a.ident().schema().cmp(&b.ident().schema()));
    let mut cleaned = Vec::default();
    for table in tables {
        if table.ident().name() == "_welds_migrations" {
            continue;
        }
        if table.ident().name() == "sqlite_sequence" {
            continue;
        }
        cleaned.push(table);
    }
    Ok(cleaned)
}

pub(crate) async fn list_views() -> Result<()> {
    let tables = fetch_db_tables().await?;
    for table in tables {
        if table.ident().name() == "_welds_migrations" {
            continue;
        }
        if table.ty() == DataType::View {
            let ident = table.ident();
            match ident.schema() {
                Some(schema) => println!("{}.{}", schema, ident.name()),
                None => println!("{}", ident.name()),
            }
        }
    }
    Ok(())
}

use tabled::Tabled;
#[derive(Tabled)]
struct ColumnDislay<'c> {
    #[tabled(rename = "Name")]
    name: &'c str,
    #[tabled(rename = "Type")]
    r#type: &'c str,
    #[tabled(rename = "Null")]
    null: bool,
    #[tabled(rename = "Primary Key")]
    pk: bool,
}

pub(crate) async fn describe(tablename: &str) -> Result<()> {
    let pool = welds::connections::connect_from_env().await?;
    let tables = find_all_tables(&pool).await?;
    let ident = TableIdent::parse(tablename);

    let found = tables
        .iter()
        .find(|t| t.ident().eq(&ident))
        .expect("Error: table not found");

    println!("\n\nTable: {}\n", found.ident());
    let rows = found.columns().iter().map(|c| ColumnDislay {
        name: c.name(),
        r#type: c.ty(),
        null: c.null(),
        pk: c.primary_key(),
    });

    use tabled::Table;
    use tabled::settings::Style;
    let table = Table::new(rows).with(Style::psql()).to_string();
    println!("{}\n", table);

    Ok(())
}
