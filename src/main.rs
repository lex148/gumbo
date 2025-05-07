use clap_complete::CompleteEnv;

pub(crate) mod action;
pub(crate) mod change;
pub(crate) mod cli;
pub(crate) mod command_handlers;
pub(crate) mod errors;
pub(crate) mod fields;
pub(crate) mod names;
mod setup_shell;
pub(crate) mod templates;

fn main() {
    // Read .env file and setup logging
    if let Err(err) = dotenvy::dotenv() {
        match err {
            dotenvy::Error::Io(_) => {}
            _ => eprintln!("DOTENV: {:?}", err),
        }
    }

    if std::env::var("COMPLETE").is_ok() {
        cli::flag_is_autocomplete();
        CompleteEnv::with_factory(cli::build_cli).complete()
    }

    // make sure the shell is setup. Ignore any failures
    let _ = setup_shell::setup_completions();

    // send the command to its command handler
    let arg = cli::build_cli().get_matches();
    match &arg.subcommand() {
        Some(("init", sub_m)) => command_handlers::init::run(sub_m),
        Some(("generate", sub_m)) => command_handlers::generate::run(sub_m),
        Some(("convert", sub_cmd)) => command_handlers::convert::run(sub_cmd),
        Some(("db", sub_m)) => command_handlers::database::run(sub_m),
        _ => unreachable!(),
    }
}
