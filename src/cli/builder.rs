use std::ffi::OsString;
use std::path::PathBuf;

use clap::{arg, Command};

pub fn cli() -> Command {
    Command::new("borg")
        .about("a wannabe ros2 command line tool replacer")
        .author("bresilla <trim.bresilla@gmail.com>")
        .version("1.0")
        .long_about("A ROS2 command line tool replacer that aims to be more user friendly and more powerful. It is written in Rust and is still in development.")
        // .arg(arg!(--color <WHEN>)
        //     .value_parser(["always", "auto", "never"])
        //     .num_args(0..=1)
        //     .require_equals(true)
        //     .default_value("auto")
        //     .default_missing_value("always"))
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .disable_help_subcommand(true)
        .subcommand(
            Command::new("mon")
                .about("Various monitor commands")
                .subcommand_required(true)
                .arg_required_else_help(true)
            .subcommand(
                Command::new("action")
                    .about("Various action commands")
                    .subcommand_required(true)
                    .arg_required_else_help(true)
            )
            .subcommand(
                Command::new("topic")
                    .about("Various topic commands")
                    .subcommand_required(true)
                    .arg_required_else_help(true)
            )
            .subcommand(
                Command::new("service")
                    .about("Various service commands")
                    .subcommand_required(true)
                    .arg_required_else_help(true)
            )
            .subcommand(
                Command::new("param")
                    .about("Various param commands")
                    .subcommand_required(true)
                    .arg_required_else_help(true)
            )
            .subcommand(
                Command::new("node")
                    .about("Various node commands")
                    .subcommand_required(true)
                    .arg_required_else_help(true)
            )
        )
        .subcommand(
            Command::new("run")
                .about("Various run commands")
                .subcommand_required(true)
                .arg_required_else_help(true)
        )
        .subcommand(
            Command::new("pkg")
                .about("Various package commands")
                .subcommand_required(true)
                .arg_required_else_help(true)
        )
        .subcommand(
            Command::new("bag")
                .about("Various rosbag commands")
                .subcommand_required(true)
                .arg_required_else_help(true)
        )
}