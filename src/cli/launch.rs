use clap::{arg, Command};

use crate::cli::utils::aly;

pub fn cmd(subs: bool, _logo: bool) -> Command {
    Command::new("launch")
        .about(aly("l", subs) + "Launch a launch file")
        .aliases(&["l"])
        .subcommand_required(true)
        .arg_required_else_help(true)
        .arg(
            arg!(<PACKAGE_NAME> "Name of the ROS package which contains the launch file")
            .required(true)
        )
        .arg(
            arg!(<LAUNCH_FILE_NAME> "Name of the launch file")
            .required(true)
        )
        .arg(
            arg!(<LAUNCH_ARGUMENTS> "Arguments to the launch file; '<name>:=<value>' (for duplicates, last one wins)")
            .required(true)
        )
        .arg(arg!(-n --noninteractive "Run the launch system non-interactively, with no terminal associated"))
        .arg(arg!(-d --debug "Put the launch system in debug mode, provides more verbose output."))
        .arg(arg!(-p --print "Print the launch description to the console without launching it."))
        .arg(arg!(-s --show_args "Show arguments that may be given to the launch file."))
        .arg(arg!(-a --show_all "Show all launched subprocesses' output"))
        .arg(arg!(--launch_prefix <LAUNCH_PREFIX> "Prefix command before executables (e.g. --launch-prefix 'xterm -e gdb -ex run --args')."))
        .arg(arg!(--launch_prefix_filter <LAUNCH_PREFIX_FILTER> "Regex pattern for executable filtering with --launch-prefix."))
}