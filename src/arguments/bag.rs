use clap::Command;

pub fn cmd() -> Command {
    Command::new("bag")
        .about("Various rosbag subcommands")
        .aliases(&["b"])
        .subcommand_required(true)
        .arg_required_else_help(true)
}