use clap::{arg, Command};

use crate::cli::utils::aly;

pub fn cmd(subs: bool, _logo: bool) -> Command {
    Command::new("action")
        .about(aly("a", subs) + "Various action subcommands")
        .aliases(&["a", "act"])
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("info")
            .about(" Print information about an action")
            .aliases(["i", "show"])
            .arg_required_else_help(true)
            .arg(
                arg!(<ACTION_NAME> "Name of the ROS action to get info (e.g. '/fibonacci')")
                .required(true)
            )
            .arg( arg!(-t --types "Additionally show the action type") )
            .arg( arg!(-c --count "Only display the number of action clients and action servers") )
        )
        .subcommand(
            Command::new("list")
            .about("List all actions")
            .aliases(["l", "ls"])
            .arg_required_else_help(true)
            .arg(arg!(-t --show_types "Additionally show the topic type"))
            .arg(arg!(-c --count_topics "Only display the number of topics discovered"))
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