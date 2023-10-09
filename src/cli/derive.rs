use clap::{Parser, Subcommand};

extern crate getset;
use getset::{CopyGetters, Getters, MutGetters, Setters};
#[derive(Parser, CopyGetters, Getters, MutGetters, Setters)]
#[getset(get = "pub", set = "pub", get_mut = "pub")]
#[command(author, version, about, long_about = None)]
#[command(name = "Lexo")]
#[command(author = "bresilla <trim.bresilla@gmail.com>")]
#[command(version = "1.0")]
#[command(about = "a wannabe ros2 command line tool replacer")]
#[command(before_help = "
  ▄▄▄         ▄▄▄         ▄▄▄               ▄▄▟███████▙▄▄      
▟█████▙     ▟█████▙     ▟█████▙           ▟███████████████▙   
▜█████▛     ▜█████▛     ▜█████▛         ▟████▛         ▜████▙ 
  ▀▀▀         ▀▀▀         ▀▀▀           ████             ▜███▙
                                                         ▟████
                                                        ▟███▛
  ▄▄▄         ▄▄▄         ▄▄▄                         ▟████▛ 
▟█████▙     ▟█████▙     ▟█████▙                     ▟████▛   
▜█████▛     ▜█████▛     ▜█████▛                   ▟████▛    
  ▀▀▀         ▀▀▀         ▀▀▀                   ▟████▛      
                                              ▟████▛       
                                            ▟████▛      
  ▄▄▄         ▄▄▄         ▄▄▄             ▟████▛            
▟█████▙     ▟█████▙     ▟█████▙          ▟████▛
▜█████▛     ▜█████▛     ▜█████▛         ▟█████████████████████
  ▀▀▀         ▀▀▀         ▀▀▀           ██████████████████████
" )]
#[command(long_about = "A ROS2 command line tool replacer that aims to be more user friendly and more powerful. It is written in Rust and is still in development.")]
// #[clap(disable_help_flag = true)]
// #[clap(disable_version_flag =true)]
#[clap(disable_help_subcommand = true)]
// #[clap(disable_version_subcommand = true)]
#[clap(disable_colored_help = false)]
#[clap(color = clap::ColorChoice::Always)]
#[clap(arg_required_else_help = true)]
#[clap(subcommand_required = true)]
#[clap(allow_external_subcommands = true)]
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
#[command(arg_required_else_help = true)]
pub enum Commands {
    #[command(name = "action")]
    #[command(about = "various action commands")]
    #[command(aliases = &["a"])]
    #[command(arg_required_else_help = true)]
    Action {
        #[command(subcommand)]
        subcommand: Option<ActionSubcommand>,
    },
    #[command(name = "topic")]
    #[command(about = "various topic commands")]
    Topic {
        #[command(subcommand)]
        subcommand: Option<TopicSubcommand>,
    },
    #[command(name = "service")]
    #[command(about = "various service commands")]
    Service {
        #[command(subcommand)]
        subcommand: Option<ServiceSubcommand>,
    },
    #[command(name = "param")]
    #[command(about = "various param commands")]
    Param {
        #[command(subcommand)]
        subcommand: Option<ParamSubcommand>,
    },
    #[command(name = "node")]
    #[command(about = "various node commands")]
    Node {
        #[command(subcommand)]
        subcommand: Option<NodeSubcommand>,
    },
    #[command(name = "interface")]
    #[command(about = "various interface commands\n")]
    Interface {
        #[command(subcommand)]
        subcommand: Option<InterfaceSubcommand>,
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
        #[command(subcommand)]
        subcommand: Option<LaunchSubcommand>,
    },



    #[command(name = "bag")]
    #[command(about = "Various rosbag commands")]
    Bag {
        #[command(subcommand)]
        subcommand: Option<BagSubcommand>,
    },


    #[command(name = "pack")]
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
    #[command(name = "create")]
    #[command(about = "various param commands")]
    Param {
        #[arg(short, long)]
        list: bool,
    },
}

#[derive(Subcommand)]
pub enum LaunchSubcommand {
    #[command(name = "param")]
    #[command(about = "various param commands")]
    Param {
        #[arg(short, long)]
        list: bool,
    },
}

#[derive(Subcommand)]
pub enum ActionSubcommand {
    #[command(name = "param")]
    #[command(about = "various param commands")]
    Param {
        #[arg(short, long)]
        list: bool,
    },
}

#[derive(Subcommand)]
pub enum TopicSubcommand {
    #[command(name = "param")]
    #[command(about = "various param commands")]
    Param {
        #[arg(short, long)]
        list: bool,
    },
}

#[derive(Subcommand)]
pub enum ServiceSubcommand {
    #[command(name = "param")]
    #[command(about = "various param commands")]
    Param {
        #[arg(short, long)]
        list: bool,
    },
}

#[derive(Subcommand)]
pub enum ParamSubcommand {
    #[command(name = "param")]
    #[command(about = "various param commands")]
    Param {
        #[arg(short, long)]
        list: bool,
    },
}

#[derive(Subcommand)]
pub enum NodeSubcommand {
    #[command(name = "param")]
    #[command(about = "various param commands")]
    Param {
        #[arg(short, long)]
        list: bool,
    },
}

#[derive(Subcommand)]
pub enum InterfaceSubcommand {
    #[command(name = "param")]
    #[command(about = "various param commands")]
    Param {
        #[arg(short, long)]
        list: bool,
    },
}