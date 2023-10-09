use clap::{arg, Command, builder::{styling, Str}};
use colored::Colorize;

pub fn wrapped(letter: &str) -> String {
    let mut wrapped = "[".red().to_string();
    wrapped.push_str(&letter.red().bold().to_string());
    wrapped.push_str(&"]".red().to_string());
    wrapped.push_str(&"  ".to_string());
    wrapped
}

pub fn cli() -> Command {
    let styles = styling::Styles::styled()
    .header(styling::AnsiColor::Red.on_default() | styling::Effects::BOLD)
    .usage(styling::AnsiColor::Red.on_default() | styling::Effects::BOLD)
    .literal(styling::AnsiColor::Green.on_default() | styling::Effects::BOLD)
    .placeholder(styling::AnsiColor::Cyan.on_default());
    Command::new("borg")
        .before_help("
  ▄▄▄   ".red().to_string().to_owned()+"     ▄▄▄   ".red().to_string().as_str() + "     ▄▄▄     ".blue().to_string().as_str() + " 
▟█████▙ ".red().to_string().as_str() + "   ▟█████▙ ".red().to_string().as_str() + "   ▟█████▙   ".blue().to_string().as_str() + "   
▜█████▛ ".red().to_string().as_str() + "   ▜█████▛ ".red().to_string().as_str() + "   ▜█████▛   ".blue().to_string().as_str() + "   
  ▀▀▀   ".red().to_string().as_str() + "     ▀▀▀   ".red().to_string().as_str() + "     ▀▀▀     ".blue().to_string().as_str() + "   
  ▄▄▄   ".red().to_string().as_str() + "     ▄▄▄   ".blue().to_string().as_str() + "     ▄▄▄     ".red().to_string().as_str() + "   
▟█████▙ ".red().to_string().as_str() + "   ▟█████▙ ".blue().to_string().as_str() + "   ▟█████▙   ".red().to_string().as_str() + "   
▜█████▛ ".red().to_string().as_str() + "   ▜█████▛ ".blue().to_string().as_str() + "   ▜█████▛   ".red().to_string().as_str() + "   
  ▀▀▀   ".red().to_string().as_str() + "     ▀▀▀   ".blue().to_string().as_str() + "     ▀▀▀     ".red().to_string().as_str() + "   
  ▄▄▄   ".red().to_string().as_str() + "     ▄▄▄   ".red().to_string().as_str() + "     ▄▄▄     ".blue().to_string().as_str() + "   
▟█████▙ ".red().to_string().as_str() + "   ▟█████▙ ".red().to_string().as_str() + "   ▟█████▙   ".blue().to_string().as_str() + "   
▜█████▛ ".red().to_string().as_str() + "   ▜█████▛ ".red().to_string().as_str() + "   ▜█████▛   ".blue().to_string().as_str() + "   
  ▀▀▀   ".red().to_string().as_str() + "     ▀▀▀   ".red().to_string().as_str() + "     ▀▀▀     ".blue().to_string().as_str())
        .styles(styles)
        .about("a wannabe ros2 command line tool replacer")
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
        .allow_external_subcommands(true)
        .disable_help_subcommand(true)
        .subcommand(
            Command::new("action")
                .about(wrapped("a") + "Various action commands")
                .aliases(&["a"])
                .subcommand_required(true)
                .arg_required_else_help(true)
                .subcommand(
                    Command::new("info")
                    .about(" Print information about an action")
                    .arg_required_else_help(true)
                )
                .subcommand(
                    Command::new("list")
                    .about("List all actions")
                    .arg_required_else_help(true)
                )
                .subcommand(
                    Command::new("send_goal")
                    .about("Send a goal to an action server")
                    .arg_required_else_help(true)
                )
        )
        .subcommand(
            Command::new("topic")
                .about(wrapped("t") + "Various topic commands")
                .aliases(&["t"])
                .subcommand_required(true)
                .arg_required_else_help(true)
                .subcommand(
                    Command::new("echo")
                    .about("Print messages from topic to screen")
                    .arg_required_else_help(true)
                )
                .subcommand(
                    Command::new("hz")
                    .about("Display publishing rate of topic")
                    .arg_required_else_help(true)
                )
                .subcommand(
                    Command::new("info")
                    .about("Print information about a topic")
                    .arg_required_else_help(true)
                )
                .subcommand(
                    Command::new("list")
                    .about("Output a list of available topics")
                    .arg_required_else_help(true)
                )
                .subcommand(
                    Command::new("pub")
                    .about("Publish a message to a topic")
                    .arg_required_else_help(true)
                )
                .subcommand(
                    Command::new("type")
                    .about("Print a topic's type")
                    .arg_required_else_help(true)
                )
                .subcommand(
                    Command::new("bw")
                    .about("Display bandwidth used by topic")
                    .arg_required_else_help(true)
                )
                .subcommand(
                    Command::new("find")
                    .about("Output a list of available topics of a given type")
                    .arg_required_else_help(true)
                )
                .subcommand(
                    Command::new("delay")
                    .about("Display delay of topic from timestamp in header")
                )
        )
        .subcommand(
            Command::new("service")
                .about(wrapped("s") + "Various service commands")
                .aliases(&["s"])
                .subcommand_required(true)
                .arg_required_else_help(true)
                .subcommand(
                    Command::new("call")
                    .about("Call a service")
                    .arg_required_else_help(true)
                )
                .subcommand(
                    Command::new("find")
                    .about("Output a list of available services of a given type")
                    .arg_required_else_help(true)
                )
                .subcommand(
                    Command::new("list")
                    .about("Output a list of available services")
                    .arg_required_else_help(true)
                )
                .subcommand(
                    Command::new("type")
                    .about("Print a service's type")
                    .arg_required_else_help(true)
                )
        )
        .subcommand(
            Command::new("param")
                .about(wrapped("p") + "Various param commands")
                .aliases(&["p"])
                .subcommand_required(true)
                .arg_required_else_help(true)
                .subcommand(
                    Command::new("get")
                    .about("Get a parameter value")
                    .arg_required_else_help(true)
                )
                .subcommand(
                    Command::new("list")
                    .about("Output a list of available parameters")
                    .arg_required_else_help(true)
                )
                .subcommand(
                    Command::new("set")
                    .about("Set a parameter value")
                    .arg_required_else_help(true)
                )
                .subcommand(
                    Command::new("dump")
                    .about("Dump all parameters to a file")
                    .arg_required_else_help(true)
                )
                .subcommand(
                    Command::new("delete")
                    .about("Delete a parameter")
                    .arg_required_else_help(true)
                )
                .subcommand(
                    Command::new("describe")
                    .about("Describe a parameter")
                    .arg_required_else_help(true)
                )
                .subcommand(
                    Command::new("load")
                    .about("Load parameters from a file")
                    .arg_required_else_help(true)
                )
        )
        .subcommand(
            Command::new("node")
                .about(wrapped("n") + "Various node commands")
                .aliases(&["n"])
                .subcommand_required(true)
                .arg_required_else_help(true)
                .subcommand(
                    Command::new("info")
                    .about("Print information about a node")
                    .arg_required_else_help(true)
                )
                .subcommand(
                    Command::new("list")
                    .about("List all nodes")
                    .arg_required_else_help(true)
                )
        )
        .subcommand(
            Command::new("interface")
                .about(wrapped("i") + "Various interface commands")
                .aliases(&["i"])
                .subcommand_required(true)
                .arg_required_else_help(true)
                .subcommand(
                    Command::new("list")
                    .about("List all interface types available")
                    .arg_required_else_help(true)
                )
                .subcommand(
                    Command::new("package")
                    .about("Output a list of available interface types within one package")
                    .arg_required_else_help(true)
                )
                .subcommand(
                    Command::new("packages")
                    .about("Output a list of available interface types within all packages")
                    .arg_required_else_help(true)
                )
                .subcommand(
                    Command::new("show")
                    .about("Show the interface definition for a given type")
                    .arg_required_else_help(true)
                )
                .subcommand(
                    Command::new("proto")
                    .about("Output an interface prototype")
                    .arg_required_else_help(true)
                )
        )



        .subcommand(
            Command::new("run")
                .about(wrapped("r") + "Run an executable")
                .aliases(&["r"])
                .subcommand_required(true)
                .arg_required_else_help(true)
        )
        .subcommand(
            Command::new("launch")
                .about(wrapped("l") + "Launch a launch file\n")
                .aliases(&["l"])
                .subcommand_required(true)
                .arg_required_else_help(true)
        )

        .subcommand(
            Command::new("pkg")
                .about("Various package commands")
                .subcommand_required(true)
                .arg_required_else_help(true)
                .subcommand(
                    Command::new("create")
                    .about("Create a new package")
                    .arg_required_else_help(true)
                )
                .subcommand(
                    Command::new("list")
                    .about("List all packages")
                    .arg_required_else_help(true)
                )
                .subcommand(
                    Command::new("info")
                    .about("Print information about a package")
                    .arg_required_else_help(true)
                )
                .subcommand(
                    Command::new("prefix")
                    .about("Print the prefix path of a package")
                    .arg_required_else_help(true)
                )
                .subcommand(
                    Command::new("share")
                    .about("Print the share path of a package")
                    .arg_required_else_help(true)
                )
                .subcommand(
                    Command::new("executables")
                    .about("Print the executables of a package")
                    .arg_required_else_help(true)
                )
                .subcommand(
                    Command::new("dependencies")
                    .about("Print the dependencies of a package")
                    .arg_required_else_help(true)
                )
                .subcommand(
                    Command::new("xml")
                    .about("Print the package.xml of a package")
                    .arg_required_else_help(true)
                )
                .subcommand(
                    Command::new("build")
                    .about("Build a package")
                    .arg_required_else_help(true)
                )
        )

        .subcommand(
            Command::new("bag")
                .about("Various rosbag commands")
                .subcommand_required(true)
                .arg_required_else_help(true)
        )
}