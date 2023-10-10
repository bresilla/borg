use clap::{arg, Command};

use crate::cli::utils::aly;

pub fn cmd(subs: bool, _logo: bool) -> Command {
    Command::new("run")
        .about(aly("r", subs) + "Run an executable")
        .aliases(&["r"])
        .subcommand_required(true)
        .arg_required_else_help(true)
        .arg(
            arg!(<PACKAGE_NAME> "Name of the ROS package to run (e.g. 'demo_nodes_cpp')")
            .required(true)
        )
        .arg(
            arg!(<EXECUTABLE_NAME> "Name of the ROS executable to run (e.g. 'talker')")
            .required(true)
        )
        .arg(
            arg!(<ARGV> "Pass arbitrary arguments to the executable (e.g. '__log_level:=debug')")
            .required(true)
        )
        .arg(arg!(--prefix <PREFIX> "Prefix command, which should go before the executable (e.g. --prefix 'gdb -ex run --args')"))
}