use clap::Command;

use crate::cli::utils::aly;

pub fn cmd(subs: bool, _logo: bool) -> Command {
    Command::new("middleware")
        .about(aly("m", subs) + "Various middleware subcommands [WIP]")
        .aliases(&["m"])
        .subcommand_required(true)
        .arg_required_else_help(true)
}