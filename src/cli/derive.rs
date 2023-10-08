use clap::{Parser, Subcommand};
use std::path::PathBuf;

extern crate getset;
use getset::{CopyGetters, Getters, MutGetters, Setters};

#[derive(Parser, CopyGetters, Getters, MutGetters, Setters)]
#[getset(get = "pub", set = "pub", get_mut = "pub")]
#[command(author, version, about, long_about = None)]
#[command(name = "Lexo")]
#[command(author = "bresilla <trim.bresilla@gmail.com>")]
#[command(version = "1.0")]
#[command(about = "a wannabe ros2 command line tool replacer")]
#[command(long_about = "A ROS2 command line tool replacer that aims to be more user friendly and more powerful. It is written in Rust and is still in development.")]
// #[clap(disable_help_flag = true)]
// #[clap(disable_version_flag =true)]
#[clap(disable_help_subcommand = true)]
// #[clap(disable_version_subcommand = true)]
#[clap(disable_colored_help = false)]
#[clap(color = clap::ColorChoice::Always)]
#[clap(arg_required_else_help = true)]
pub struct Cli {
    /// Set the config file path
    // #[arg(long, value_name = "FILE")]
    // config: Option<PathBuf>,

    /// Set the library file path
    // #[arg(long, value_name = "FILE")]
    // library: Option<PathBuf>,

    #[command(subcommand)]
    command: Option<Commands>,
}


#[derive(Subcommand)]
pub enum Commands {
    #[command(name = "action")]
    #[command(about = "various action commands")]
    //make a better about
    Action {
        #[arg(short, long)]
        list: bool,
    },
    #[command(name = "topic")]
    #[command(about = "various topic commands")]
    Topic {
        #[arg(short, long)]
        list: bool,
    },
    #[command(name = "service")]
    #[command(about = "various service commands")]
    Service {
        #[arg(short, long)]
        list: bool,
    },
    #[command(name = "param")]
    #[command(about = "various param commands")]
    Param {
        #[arg(short, long)]
        list: bool,
    },
    #[command(name = "node")]
    #[command(about = "various node commands")]
    Node {
        #[arg(short, long)]
        list: bool,
    },
    #[command(name = "interface")]
    #[command(about = "various interface commands\n")]
    Interface {
        #[arg(short, long)]
        list: bool,
    },


    #[command(name = "run")]
    #[command(about = "Various run commands")]
    Run {
        #[command(subcommand)]
        subcommand: Option<RunSubcommand>,
    },
    #[command(name = "launch")]
    #[command(about = "Various launch commands\n")]
    Launch {
        #[arg(short, long)]
        list: bool,
    },



    #[command(name = "bag")]
    #[command(about = "Various rosbag commands")]
    Bag {
        #[command(subcommand)]
        subcommand: Option<BagSubcommand>,
    },


    #[command(name = "pkg")]
    #[command(about = "Various package commands")]
    Pkg {
        #[command(subcommand)]
        subcommand: Option<PkgSubcommand>,
    },
}

#[derive(Subcommand)]
pub enum BagSubcommand {
    #[command(name = "param")]
    #[command(about = "various param commands")]
    Param {
        #[arg(short, long)]
        list: bool,
    },
}

#[derive(Subcommand)]
pub enum PkgSubcommand {
    #[command(name = "param")]
    #[command(about = "various param commands")]
    Param {
        #[arg(short, long)]
        list: bool,
    },
}

#[derive(Subcommand)]
pub enum RunSubcommand {
    #[command(name = "param")]
    #[command(about = "various param commands")]
    Param {
        #[arg(short, long)]
        list: bool,
    },
}