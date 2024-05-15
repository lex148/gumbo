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
    let arg = cli::Cli::parse();

    // send the command to its command handler
    match &arg.command {
        RootCommand::Init { path } => command_handlers::init::run(path),
        RootCommand::Generate { sub_cmd } => command_handlers::generate::run(sub_cmd),
        RootCommand::Convert { sub_cmd } => command_handlers::convert::run(sub_cmd),
    }
}
