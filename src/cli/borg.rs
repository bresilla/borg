use clap::{Command, builder::styling};
use colored::Colorize;
use crate::cli::*;

pub fn aly(letter: &str) -> String {
    let mut wrapped = "[".bright_green().to_string();
    wrapped.push_str(&letter.bright_green().italic().to_string());
    wrapped.push_str(&"]".bright_green().to_string());
    wrapped.push_str(&"  ".to_string());
    wrapped
}

const ABOUT_STR: &str = "a wannabe ros2 command line tool alternative";

pub fn cli(logo: bool) -> Command {
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


    let help_str: String = " ".to_string().to_owned()+"
    
"+ &ABOUT_STR +"\n
Usage:".bright_blue().bold().to_string().as_str()+"  borg".bright_green().bold().to_string().as_str()+" <COMMAND>".green().to_string().as_str()+"

Monotor Commands:".bright_blue().bold().to_string().as_str()+"
  action".bright_green().bold().to_string().as_str() + "      "+&aly("a")+"  Various action subcommands".bright_white().to_string().as_str()+"
  topic".bright_green().bold().to_string().as_str() + "       "+&aly("t")+"  Various topic subcommands".bright_white().to_string().as_str()+"
  service".bright_green().bold().to_string().as_str() + "     "+&aly("s")+"  Various service subcommands".bright_white().to_string().as_str()+"
  param".bright_green().bold().to_string().as_str() + "       "+&aly("p")+"  Various param subcommands".bright_white().to_string().as_str()+"
  node".bright_green().bold().to_string().as_str() + "        "+&aly("n")+"  Various node subcommands".bright_white().to_string().as_str()+"
  interface".bright_green().bold().to_string().as_str() + "   "+&aly("i")+"  Various interface subcommands".bright_white().to_string().as_str()+"
  frame".bright_green().bold().to_string().as_str() + "       "+&aly("f")+"  Various transforms subcommands [WIP]".bright_white().to_string().as_str()+"

Workspace Commands:".bright_blue().bold().to_string().as_str()+"
  run".bright_green().bold().to_string().as_str() + "         "+&aly("r")+"  Run an executable".bright_white().to_string().as_str()+"
  launch".bright_green().bold().to_string().as_str() + "      "+&aly("l")+"  Launch a launch file".bright_white().to_string().as_str()+"
  work".bright_green().bold().to_string().as_str() + "        "+&aly("w")+"  Various workspace subcommands".bright_white().to_string().as_str()+"

Utilities Commands:".bright_blue().bold().to_string().as_str()+"     
  bag".bright_green().bold().to_string().as_str() + "         "+&aly("b")+"  Various rosbag subcommands".bright_white().to_string().as_str()+"
  daemon".bright_green().bold().to_string().as_str() + "      "+&aly("d")+"  Deamon and bridge subcommands [WIP]".bright_white().to_string().as_str()+"
  middleware".bright_green().bold().to_string().as_str() + "  "+&aly("m")+"  Various middleware subcommands [WIP]".bright_white().to_string().as_str();

    let styles = styling::Styles::styled()
        .header(styling::AnsiColor::Blue.on_default() | styling::Effects::BOLD)
        .usage(styling::AnsiColor::Blue.on_default() | styling::Effects::BOLD)
        .literal(styling::AnsiColor::Green.on_default() | styling::Effects::BOLD)
        .error(styling::AnsiColor::Red.on_default() | styling::Effects::BOLD)
        .placeholder(styling::AnsiColor::Green.on_default());

    Command::new("borg")
        .styles(styles)
        .about(&ABOUT_STR) 
        .author("bresilla <trim.bresilla@gmail.com>")
        .version("1.0")
        .long_about("A ROS2 command line tool replacer that aims to be more user friendly and more powerful. [ALPHA STATE]")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(false)
        .disable_help_subcommand(true)
        .override_help(logo_str + &help_str)
        .subcommand(action::cmd())
        .subcommand(topic::cmd())
        .subcommand(service::cmd())
        .subcommand(param::cmd())
        .subcommand(node::cmd())
        .subcommand(interface::cmd())
        .subcommand(frame::cmd())
        .subcommand(run::cmd())
        .subcommand(launch::cmd())
        .subcommand(work::cmd())
        .subcommand(bag::cmd())
        .subcommand(daemon::cmd())
        .subcommand(middleware::cmd())
}