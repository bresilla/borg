use clap::{arg, Command};

pub fn cmd() -> Command {
    Command::new("service")
        .about("Various service subcommands")
        .aliases(&["s", "ser"])
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("call")
            .about("Call a service")
            .aliases(["c", "invoke"])
            .arg_required_else_help(true)
            .arg(
                arg!(<SERVICE_NAME> "Name of the ROS service to call to (e.g. '/add_two_ints')")
                .required(true)
            )
            .arg(
                arg!(<SERVICE_TYPE> "Type of the ROS service (e.g. 'std_srvs/srv/Empty')")
                .required(true)
            )
            .arg(
                arg!(<VALUES> "Values to fill the service request with in YAML format")
                .required(true)
            )
            .arg(arg!(-r --rate <RATE> "Repeat the call at a specific rate in Hz"))
        )
        .subcommand(
            Command::new("find")
            .about("Output a list of available services of a given type")
            .aliases(["f", "lookup", "search"])
            .arg_required_else_help(true)
            .arg(
                arg!(<SERVICE_TYPE> "Name of the ROS service type to filter for (e.g. 'std_srvs/srv/Empty')")
                .required(true)
            )
            .arg(arg!(-c --count_services "Only display the number of services discovered"))
            .arg(arg!(--include_hidden_services "Consider hidden services as well"))
        )
        .subcommand(
            Command::new("list")
            .about("Output a list of available services")
            .aliases(["l", "ls"])
            .arg_required_else_help(true)
            .arg(arg!(--spin_time <SPIN_TIME> "Spin time for discovery (if daemon not used)"))
            .arg(arg!(-s --use_sim_time "Enable ROS sim time"))
            .arg(arg!(--no_daemon "Don't spawn or use a running daemon"))
            .arg(arg!(-t --show_types "Show service type"))
            .arg(arg!(-c --count_services "Display only the number of services discovered"))
            .arg(arg!(--include_hidden_services "Include hidden services"))
        )
        .subcommand(
            Command::new("kind")
            .about("Print a service's type")
            .aliases(["k", "type"])
            .arg_required_else_help(true)
            .arg(
                arg!(<SERVICE_NAME> "Name of the ROS service to get type (e.g. '/add_two_ints')")
                .required(true)
            )
        )
}