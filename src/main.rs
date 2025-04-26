use clap::Parser;

use crate::cli::RootCommand;

pub(crate) mod action;
pub(crate) mod change;
pub(crate) mod cli;
pub(crate) mod command_handlers;
pub(crate) mod errors;
pub(crate) mod fields;
pub(crate) mod names;
pub(crate) mod templates;

fn main() {
    // Read .env file and setup logging
    if let Err(err) = dotenvy::dotenv() {
        match err {
            dotenvy::Error::Io(_) => {}
            _ => eprintln!("DOTENV: {:?}", err),
        }
    }

    let arg = cli::Cli::parse();

    // send the command to its command handler
    match &arg.command {
        RootCommand::Init { path, welds_only } => command_handlers::init::run(path, *welds_only),
        RootCommand::Generate { sub_cmd } => command_handlers::generate::run(sub_cmd),
        RootCommand::Convert { sub_cmd } => command_handlers::convert::run(sub_cmd),
        RootCommand::Database { sub_cmd } => command_handlers::database::run(sub_cmd),
    }
}
