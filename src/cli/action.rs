use clap::{arg, Command, Arg, ArgAction};

pub fn cmd() -> Command {
    Command::new("action")
        .about("Various action subcommands")
        .aliases(&["a", "act"])
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("info")
            .about(" Print information about an action")
            .aliases(["i", "show"])
            .arg_required_else_help(true)
            .arg(
                Arg::new("action_name")
                .help("Name of the ROS action to get info (e.g. '/fibonacci')")
                .required(true)
                .value_name("ACTION_NAME")
            )
            .arg(
                Arg::new("show_types")
                .short('t')
                .long("show-types")
                .aliases(&["show_types", "types"])
                .help("Additionally show the action type")
                .action(ArgAction::SetTrue)
                .conflicts_with("count_actions")
            )
            .arg(
                Arg::new("count_actions")
                .short('c')
                .long("count-actions")
                .aliases(&["count_actions", "count"])
                .help("Only display the number of actions discovered")
                .action(ArgAction::SetTrue)
                .conflicts_with("show_types")
            )
        )
        .subcommand(
            Command::new("list")
            .about("List all actions")
            .aliases(["l", "ls"])
            .arg(
                Arg::new("show_types")
                .short('t')
                .long("show-types")
                .aliases(&["show_types", "types"])
                .help("Additionally show the action type")
                .action(ArgAction::SetTrue)
                .conflicts_with("count_actions")
            )
            .arg(
                Arg::new("count_actions")
                .short('c')
                .long("count-actions")
                .aliases(&["count_actions", "count"])
                .help("Only display the number of actions discovered")
                .action(ArgAction::SetTrue)
                .conflicts_with("show_types")
            )

        )
        .subcommand(
            Command::new("goal")
            .about("Send a goal to an action server")
            .aliases(["g", "send_goal"])
            .arg_required_else_help(true)
            .arg(
                arg!(<ACTION_NAME> "Name of the ROS action (e.g. '/fibonacci')")
                .required(true)
            )
            .arg(
                arg!(<ACTION_TYPE> "Type of the ROS action (e.g. 'example_interfaces/action/Fibonacci')")
                .required(true)
            )
            .arg(
                arg!(<GOAL> "Goal to send to the action server (e.g. '{order: 10}')")
                .required(true)
            )
            .arg( arg!(-f --feedback "Echo feedback messages for the goal") )
        )
}