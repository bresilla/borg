use clap::{arg, Command};

use crate::cli::utils::aly;

pub fn cmd(subs: bool, _logo: bool) -> Command {
    Command::new("interface")
        .about(aly("i", subs) + "Various interface subcommands")
        .aliases(&["i", "int"])
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("list")
            .about("List all interface types available")
            .aliases(["l", "ls"])
            .arg_required_else_help(true)
            .arg(arg!(-m --only_msgs "Print out only the message types"))
            .arg(arg!(-s --only_srvs "Print out only the service types"))
            .arg(arg!(-a --only_actions "Print out only the action types"))
        )
        .subcommand(
            Command::new("package")
            .about("Output a list of available interface types within one package")
            .aliases(["p", "pkg"])
            .arg_required_else_help(true)
            .arg(
                arg!(<PACKAGE_NAME> "Name of the ROS package to get info (e.g. 'std_msgs')")
                .required(true)
            )
        )
        .subcommand(
            Command::new("all")
            .about("Output a list of available interface types within all packages")
            .aliases(["a", "packages"])
            .arg_required_else_help(true)
            .arg(arg!(-m --only_msgs "Print out only the message types"))
            .arg(arg!(-s --only_srvs "Print out only the service types"))
            .arg(arg!(-a --only_actions "Print out only the action types"))
        )
        .subcommand(
            Command::new("show")
            .about("Show the interface definition for a given type")
            .aliases(["s", "info"])
            .arg_required_else_help(true)
            .arg(
                arg!(<TYPE> "Show an interface definition (e.g. 'example_interfaces/msg/String'). Passing '-' reads the argument from stdin (e.g. 'ros2 topic type /chatter | ros2 interface show -').")
                .required(true)
            )
            .arg(arg!(--all_comments "Show all comments, including for nested interface definitions"))
            .arg(arg!(--no_comments "Show no comments or whitespace"))
        )
        .subcommand(
            Command::new("model")
            .about("Output an interface model/prototype")
            .aliases(["m", "prototype", "proto"])
            .arg_required_else_help(true)                                                                                                                                                                                                                      
            .arg(
                arg!(<TYPE> "Show an interface definition (e.g. 'example_interfaces/msg/String')")
                .required(true)
            )
            .arg(arg!(--no_quotes "if true output has no outer quotes"))               
        )
}