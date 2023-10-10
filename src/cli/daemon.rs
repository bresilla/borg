use clap::Command;

use crate::cli::utils::aly;

pub fn cmd(subs: bool, _logo: bool) -> Command {
    Command::new("daemon")
        .about(aly("d", subs) + "Deamon and bridge subcommands [WIP]")
        .aliases(&["d"])
        .subcommand_required(true)
        .arg_required_else_help(true)
}