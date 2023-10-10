use clap::Command;

use crate::cli::utils::aly;

pub fn cmd(subs: bool, _logo: bool) -> Command {
    Command::new("bag")
        .about(aly("b", subs) + "Various rosbag subcommands")
        .aliases(&["b"])
        .subcommand_required(true)
        .arg_required_else_help(true)
}