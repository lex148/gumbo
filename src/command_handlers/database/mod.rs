use crate::errors::Result;
use welds::Client;
pub(crate) mod list_tables;
mod model_from_table;
use crate::change::write_to_disk;
use crate::command_handlers::generate::get_root_path;
use clap::ArgMatches;

/// Called to to crate a new gumbo project
pub fn run(args: &ArgMatches) {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_time()
        .enable_io()
        .build()
        .unwrap();
    rt.block_on(async {
        if let Err(err) = run_inner(args).await {
            eprintln!("{err}");
            std::process::exit(1);
        }
    })
}

async fn run_inner(args: &ArgMatches) -> Result<()> {
    match args.subcommand() {
        Some(("rollback", _sub_m)) => welds_rollback().await?,
        Some(("test-connection", _sub_m)) => test_connection().await?,
        Some(("list-tables", _sub_m)) => list_tables::list_tables().await?,
        Some(("list-views", _sub_m)) => list_tables::list_views().await?,
        Some(("describe", sub_m)) => {
            let table = sub_m.get_one::<String>("table").unwrap();
            list_tables::describe(table).await?;
        }
        Some(("model-from-table", sub_m)) => {
            let tables = sub_m.get_many::<String>("tables").unwrap();
            let tables: Vec<String> = tables.into_iter().cloned().collect();
            // if the table is "--all-tables", update them all
            let mut tables: Vec<String> = tables.to_vec();
            if tables.first().map(|s| s.as_str()) == Some("--all-tables") {
                let tables_def = list_tables::fetch_db_tables().await?;
                tables = tables_def.iter().map(|d| d.ident().to_string()).collect();
            }

            let root_path = get_root_path()?;
            let mut changes = Vec::default();
            for table in &tables {
                changes.push(model_from_table::run(table).await?);
            }
            for change in changes.as_slice().iter().flatten() {
                println!("FILE: {:?}", change.file());
            }
            write_to_disk(&root_path, changes.as_slice().iter().flatten())?;
            crate::command_handlers::run_rustfmt(&root_path);
            println!("Model Update Completed");
        }
        _ => unreachable!(),
    }
    //match cmd {
    //    DatabaseCommands::Rollback => welds_rollback().await?,
    //    DatabaseCommands::TestConnection => test_connection().await?,
    //    DatabaseCommands::ListTables => list_tables::list_tables().await?,
    //    DatabaseCommands::ListViews => list_tables::list_views().await?,
    //    DatabaseCommands::Describe { table } => list_tables::describe(table).await?,
    //    DatabaseCommands::ModelFromTable { tables } => {
    //        // if the table is "--all-tables", update them all
    //        let mut tables: Vec<String> = tables.to_vec();
    //        if tables.first().map(|s| s.as_str()) == Some("--all-tables") {
    //            let tables_def = list_tables::fetch_db_tables().await?;
    //            tables = tables_def.iter().map(|d| d.ident().to_string()).collect();
    //        }

    //        let root_path = get_root_path()?;
    //        let mut changes = Vec::default();
    //        for table in &tables {
    //            changes.push(model_from_table::run(table).await?);
    //        }
    //        for change in changes.as_slice().iter().flatten() {
    //            println!("FILE: {:?}", change.file());
    //        }
    //        for change in changes.as_slice().iter().flatten() {
    //            write_to_disk(&root_path, change)?;
    //        }
    //        crate::command_handlers::run_rustfmt(&root_path);
    //        println!("Model Update Completed");
    //    }
    //}
    Ok(())
}

async fn test_connection() -> Result<()> {
    let pool = welds::connections::connect_from_env().await?;
    pool.execute("SELECT 1", &[]).await?;
    println!("CONNECTED TO DATABASE");
    Ok(())
}

async fn welds_rollback() -> Result<()> {
    let trans_start = welds::connections::connect_from_env().await?;
    let name = welds::migrations::down_last(trans_start.as_ref()).await?;
    println!("Migration Down Complete: {:?}", name);
    Ok(())
}
