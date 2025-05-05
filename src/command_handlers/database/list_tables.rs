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

/// Fetch a list of table from the database.
/// common system level tables are ignored.
pub(crate) async fn fetch_db_tables() -> Result<Vec<TableDef>> {
    let pool = welds::connections::connect_from_env().await?;
    let mut tables = find_all_tables(&pool).await?;

    let ignore: Vec<TableIdent> = IGNORE_TABLES.iter().map(|t| TableIdent::parse(t)).collect();

    tables.sort_by(|a, b| a.ident().name().cmp(b.ident().name()));
    tables.sort_by(|a, b| a.ident().schema().cmp(&b.ident().schema()));

    Ok(tables
        .iter()
        .filter(|t| !ignore.contains(t.ident()))
        .cloned()
        .collect())
}

/// list of tables that will be ignored by gumbo
const IGNORE_TABLES: &[&str] = &[
    "public._welds_migrations",
    "_welds_migrations",
    "public.ar_internal_metadata",
    "metric_helpers.index_bloat",
    "metric_helpers.table_bloat",
    "metric_helpers.nearly_exhausted_sequences",
    "metric_helpers.pg_stat_statements",
    "public.pg_stat_kcaches",
    "public.pg_stat_kcache_details",
    "public.pg_stat_statements",
    "public.pg_stat_statements_infos",
    "public.pg_stat_kcache",
    "public.pg_stat_kcache_detail",
    "public.pg_stat_statements_info",
    "public.schema_migrations",
    "sqlite_sequence",
];

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
