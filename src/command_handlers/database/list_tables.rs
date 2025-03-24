use welds::detect::{find_tables, DataType};
use welds::errors::Result;
use welds::model_traits::TableIdent;

// Print out a list of tables in the database
pub(crate) fn run() -> Result<()> {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_time()
        .enable_io()
        .build()
        .unwrap();
    rt.block_on(async { list_tables_inner().await })
}

async fn list_tables_inner() -> Result<()> {
    let pool = welds::connections::connect_from_env().await?;
    let mut tables = find_tables(&pool).await?;
    tables.sort_by(|a, b| a.ident().name().cmp(b.ident().name()));
    tables.sort_by(|a, b| a.ident().schema().cmp(&b.ident().schema()));
    for table in tables {
        if table.ident().name() == "_welds_migrations" {
            continue;
        }
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

// Print out a list of tables in the database
pub(crate) fn run_views() -> Result<()> {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_time()
        .enable_io()
        .build()
        .unwrap();
    rt.block_on(async { list_views_inner().await })
}

async fn list_views_inner() -> Result<()> {
    let pool = welds::connections::connect_from_env().await?;
    let mut tables = find_tables(&pool).await?;
    tables.sort_by(|a, b| a.ident().name().cmp(b.ident().name()));
    tables.sort_by(|a, b| a.ident().schema().cmp(&b.ident().schema()));
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

// Print out a list of tables in the database
pub(crate) fn describe(tablename: &str) -> Result<()> {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_time()
        .enable_io()
        .build()
        .unwrap();
    rt.block_on(async { describe_inner(tablename).await })
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

async fn describe_inner(tablename: &str) -> Result<()> {
    let pool = welds::connections::connect_from_env().await?;
    let tables = find_tables(&pool).await?;
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

    use tabled::settings::Style;
    use tabled::Table;
    let table = Table::new(rows).with(Style::psql()).to_string();
    println!("{}\n", table);

    Ok(())
}
