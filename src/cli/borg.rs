use clap::{Command, builder::styling};
use colored::Colorize;
use crate::cli::*;


pub fn cli(subs: bool, logo: bool) -> Command {
    let logo_str: String = if logo {"
        ▄▄▄   ".bright_blue().to_string().to_owned()+"     ▄▄▄   ".bright_blue().to_string().as_str() + "     ▄▄▄     ".bright_green().to_string().as_str() + " 
      ▟█████▙ ".bright_blue().to_string().as_str() + "   ▟█████▙ ".bright_blue().to_string().as_str() + "   ▟█████▙   ".bright_green().to_string().as_str() + "   
      ▜█████▛ ".bright_blue().to_string().as_str() + "   ▜█████▛ ".bright_blue().to_string().as_str() + "   ▜█████▛   ".bright_green().to_string().as_str() + "   
        ▀▀▀   ".bright_blue().to_string().as_str() + "     ▀▀▀   ".bright_blue().to_string().as_str() + "     ▀▀▀     ".bright_green().to_string().as_str() + "   
        ▄▄▄   ".bright_blue().to_string().as_str() + "     ▄▄▄   ".bright_green().to_string().as_str() + "     ▄▄▄     ".bright_blue().to_string().as_str() + "   
      ▟█████▙ ".bright_blue().to_string().as_str() + "   ▟█████▙ ".bright_green().to_string().as_str() + "   ▟█████▙   ".bright_blue().to_string().as_str() + "   
      ▜█████▛ ".bright_blue().to_string().as_str() + "   ▜█████▛ ".bright_green().to_string().as_str() + "   ▜█████▛   ".bright_blue().to_string().as_str() + "   
        ▀▀▀   ".bright_blue().to_string().as_str() + "     ▀▀▀   ".bright_green().to_string().as_str() + "     ▀▀▀     ".bright_blue().to_string().as_str() + "   
        ▄▄▄   ".bright_blue().to_string().as_str() + "     ▄▄▄   ".bright_blue().to_string().as_str() + "     ▄▄▄     ".bright_green().to_string().as_str() + "   
      ▟█████▙ ".bright_blue().to_string().as_str() + "   ▟█████▙ ".bright_blue().to_string().as_str() + "   ▟█████▙   ".bright_green().to_string().as_str() + "   
      ▜█████▛ ".bright_blue().to_string().as_str() + "   ▜█████▛ ".bright_blue().to_string().as_str() + "   ▜█████▛   ".bright_green().to_string().as_str() + "   
        ▀▀▀   ".bright_blue().to_string().as_str() + "     ▀▀▀   ".bright_blue().to_string().as_str() + "     ▀▀▀     ".bright_green().to_string().as_str()} else { String::new() };


    let styles = styling::Styles::styled()
    .header(styling::AnsiColor::Blue.on_default() | styling::Effects::BOLD)
    .usage(styling::AnsiColor::Blue.on_default() | styling::Effects::BOLD)
    .literal(styling::AnsiColor::Green.on_default() | styling::Effects::BOLD)
    .error(styling::AnsiColor::Red.on_default() | styling::Effects::BOLD)
    .placeholder(styling::AnsiColor::Green.on_default());


    Command::new("borg")
        .before_help(&logo_str)
        .styles(styles)
        .about("a wannabe ros2 command line tool alternative") 
        .author("bresilla <trim.bresilla@gmail.com>")
        .version("1.0")
        .long_about("A ROS2 command line tool replacer that aims to be more user friendly and more powerful. [ALPHA STATE]")
        // .arg(arg!(--color <WHEN>)
        //     .value_parser(["always", "auto", "never"])
        //     .num_args(0..=1)
        //     .require_equals(true)
        //     .default_value("auto")
        //     .default_missing_value("always"))
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(false)
        .disable_help_subcommand(true)
        .subcommand(action::cmd(subs, logo))
        .subcommand(topic::cmd(subs, logo))
        .subcommand(service::cmd(subs, logo))
        .subcommand(param::cmd(subs, logo))
        .subcommand(node::cmd(subs, logo))
        .subcommand(interface::cmd(subs, logo))
        .subcommand(frame::cmd(subs, logo))

        .subcommand(run::cmd(subs, logo))
        .subcommand(launch::cmd(subs, logo))
        .subcommand(work::cmd(subs, logo))

        .subcommand(bag::cmd(subs, logo))
        .subcommand(daemon::cmd(subs, logo))
        .subcommand(middleware::cmd(subs, logo))
}