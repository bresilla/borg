use clap::{arg, Command};

pub fn cmd() -> Command {
    Command::new("node")
        .about("Various node subcommands")
        .aliases(&["n", "nod"])
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("info")
            .about("Print information about a node")
            .aliases(["i", "show"])
            .arg_required_else_help(true)
            .arg(
                arg!(<NODE_NAME> "Name of the ROS node to get info (e.g. '/talker')")
                .required(true)
            )
            .arg(arg!(--spin_time <SPIN_TIME> "Spin time for discovery (if daemon not used)"))
            .arg(arg!(--no_daemon "Don't spawn or use a running daemon"))
            .arg(arg!(--include_hidden_services "Include hidden services"))
        )
        .subcommand(
            Command::new("list")
            .about("List all nodes")
            .aliases(["l", "ls"])
            .arg_required_else_help(true)
            .arg(arg!(--spin_time <SPIN_TIME> "Spin time for discovery (if daemon not used)"))
            .arg(arg!(-a --all "Display all nodes even hidden ones"))                                                                                                                                                                                                    
            .arg(arg!(-c --count_nodes "Only display the number of nodes discovered"))
        )
}