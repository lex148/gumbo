use clap::builder::PathBufValueParser;
use clap::{Arg, ArgAction, Command};

pub(crate) fn build_cli() -> Command {
    Command::new("gumbo")
        .version(env!("CARGO_PKG_VERSION"))
        .propagate_version(true)
        .arg_required_else_help(true)
        .subcommand(init_subcommand())
        .subcommand(generate_subcommand())
        .subcommand(db_subcommand())
        .subcommand(convert_subcommand())
        .subcommand(Command::new("setup-shell").about("add gumbo auto-completion to your shell"))
}

fn init_subcommand() -> Command {
    Command::new("init")
        .about("Create a new project all setup for gumbo")
        .arg(
            Arg::new("path")
                .help("Path to where to initialize the new project")
                .required(true)
                .value_parser(PathBufValueParser::new()),
        )
        .arg(
            Arg::new("welds_only")
                .long("welds-only")
                .help("Only setup for welds (no full actix website)")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("mssql")
                .long("mssql")
                .help("Enable support for mssql")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("mysql")
                .long("mysql")
                .help("Enable support for mysql")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("postgres")
                .long("postgres")
                .help("Enable support for postgres")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("sqlite")
                .long("sqlite")
                .help("Enable support for sqlite")
                .action(ArgAction::SetTrue),
        )
}

fn generate_subcommand() -> Command {
    Command::new("generate")
        .about("Used to generate code")
        .alias("g")
        .subcommand_required(true)
        .subcommand(scaffold_subcommand())
        .subcommand(model_subcommand())
        .subcommand(migration_subcommand())
        .subcommand(controller_subcommand())
        .subcommand(env_subcommand())
}

fn scaffold_subcommand() -> Command {
    Command::new("scaffold")
        .about("Scaffold out a full model/view/controller")
        .alias("s")
        .arg(
            Arg::new("name")
                .help("Name of the resource to scaffold")
                .required(true),
        )
        .arg(
            Arg::new("fields")
                .help("List of fields for model (e.g. name:string description:text:option)")
                .required(true)
                .num_args(1..),
        )
}

fn model_subcommand() -> Command {
    Command::new("model")
        .about("Generate a model")
        .alias("m")
        .arg(
            Arg::new("name")
                .help("Name of the Model to generate")
                .required(true),
        )
        .arg(
            Arg::new("fields")
                .help("List of fields for model")
                .required(true)
                .num_args(1..),
        )
        .arg(
            Arg::new("no_migration")
                .long("no-migration")
                .help("Disable migration generation")
                .action(ArgAction::SetTrue),
        )
}

fn migration_subcommand() -> Command {
    Command::new("migration")
        .about("Generate a migration, no model attached")
        .alias("db")
        .arg(
            Arg::new("name")
                .help("Name of the Migration to generate")
                .required(true),
        )
        .arg(
            Arg::new("fields")
                .help("List of fields for table")
                .required(true)
                .num_args(1..),
        )
}

fn controller_subcommand() -> Command {
    Command::new("controller")
        .about("Generate a controller and its actions")
        .alias("c")
        .arg(
            Arg::new("name")
                .help("Name of the controller (e.g. Cars → cars_controller)")
                .required(true),
        )
        .arg(
            Arg::new("actions")
                .help("List of actions (index, show, new, create, edit, update, delete)")
                .required(true)
                .num_args(1..),
        )
        .arg(
            Arg::new("no_views")
                .long("no-views")
                .help("Disable the creation of views")
                .action(ArgAction::SetTrue),
        )
}

fn env_subcommand() -> Command {
    Command::new("env").about("Generate a .env file with all settings needed for your gumbo app")
}

fn db_subcommand() -> Command {
    Command::new("db")
        .about("Helper utils for weldsorm")
        .subcommand_required(true)
        .subcommand(
            Command::new("rollback").about("Rollback the last welds migration (migrate down)"),
        )
        .subcommand(
            Command::new("test-connection").about("Check database connectivity via DATABASE_URL"),
        )
        .subcommand(Command::new("list-tables").about("List tables in the database"))
        .subcommand(Command::new("list-views").about("List views in the database"))
        .subcommand(describe_subcommand())
        .subcommand(model_from_table_subcommand())
}

fn describe_subcommand() -> Command {
    let mut cmd = Command::new("describe");

    // inject table "command" really just tables into the auto-complete list
    if is_autocomplete() && std::env::var("DATABASE_URL").is_ok() {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_time()
            .enable_io()
            .build()
            .unwrap();
        let tables = rt.block_on(async {
            crate::command_handlers::database::list_tables::fetch_db_tables()
                .await
                .unwrap_or_default()
        });
        for name in tables.iter().map(|t| t.ident().to_string()) {
            cmd = cmd.subcommand(Command::new(name.clone()));
        }
    }

    cmd.about("Print columns and types of a table/view").arg(
        Arg::new("table")
            .help("Name of the table or view")
            .required(true),
    )
}

static mut FLAG: bool = false;
pub(crate) fn flag_is_autocomplete() {
    unsafe {
        FLAG = true;
    }
}

fn is_autocomplete() -> bool {
    unsafe { FLAG }
}

fn model_from_table_subcommand() -> Command {
    let mut cmd = Command::new("model-from-table")
        .about("Create a model from one or more database tables")
        .alias("m");

    // inject table "command" really just tables into the auto-complete list
    if is_autocomplete() && std::env::var("DATABASE_URL").is_ok() {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_time()
            .enable_io()
            .build()
            .unwrap();
        let tables = rt.block_on(async {
            crate::command_handlers::database::list_tables::fetch_db_tables()
                .await
                .unwrap_or_default()
        });
        for name in tables.iter().map(|t| t.ident().to_string()) {
            cmd = cmd.subcommand(Command::new(name.clone()));
        }
    }

    cmd.arg(
        Arg::new("tables")
            .help("List of tables to import into models")
            .required(true)
            .num_args(1..),
    )
}

fn convert_subcommand() -> Command {
    Command::new("convert")
        .about("Automatically convert your code")
        .alias("c")
        .subcommand_required(true)
        .subcommand(mod2dir_subcommand())
        .subcommand(dir2mod_subcommand())
}

fn mod2dir_subcommand() -> Command {
    Command::new("mod2dir")
        .about(r#"Convert "bla.rs" into "bla/mod.rs" folder structure"#)
        .arg(
            Arg::new("path")
                .help("Path to the file or directory")
                .required(true)
                .value_parser(PathBufValueParser::new()),
        )
}

fn dir2mod_subcommand() -> Command {
    Command::new("dir2mod")
        .about(r#"Convert "bla/mod.rs" into "bla.rs" file"#)
        .arg(
            Arg::new("path")
                .help("Path to the folder or mod.rs")
                .required(true)
                .value_parser(PathBufValueParser::new()),
        )
}
