use clap::{arg, Command, builder::styling};
use colored::Colorize;
// use std::path::PathBuf;

pub fn aly(letter: &str, show_logo: bool) -> String {
    let mut wrapped = "[ ".bright_green().to_string();
    wrapped.push_str(&letter.bright_green().bold().to_string());
    wrapped.push_str(&" ]".bright_green().to_string());
    wrapped.push_str(&"  ".to_string());
    let new_wrapped = if show_logo { wrapped } else { "".to_string() };
    new_wrapped
}

pub fn cli(subs: bool, logo: bool) -> Command {
    let logo: String = if logo {"
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
        .before_help(&logo)
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
        .subcommand(
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
        )
        .subcommand(
            Command::new("topic")
                .about(aly("t", subs) + "Various topic subcommands")
                .aliases(&["t", "top"])
                .subcommand_required(true)
                .arg_required_else_help(true)
                .subcommand(
                    Command::new("echo")
                    .about("Print messages from topic to screen")
                    .aliases(["e", "cat"])
                    .arg_required_else_help(true)
                    .arg(
                        arg!(<TOPIC_NAME> "Name of the ROS topic to echo (e.g. '/chatter')")
                        .required(true)
                    )
                    .arg(
                        arg!(<MESSAGE_TYPE> "Type of the ROS message (e.g. 'std_msgs/msg/String')")
                        .required(true)
                    )
                    .arg(arg!(--spin_time <SPIN_TIME> "Time (sec) to spin. Default: until interrupted"))
                    .arg(arg!(-s --use_sim_time "Use simulation time if available"))
                    .arg(arg!(--no_daemon "Don't spawn or use running daemon"))
                    .arg(arg!(--qos_profile <QOS_PROFILE> "QoS preset profile (default: sensor_data)"))
                    .arg(arg!(--qos_depth <QOS_DEPTH> "Queue size setting (overrides QoS profile)"))
                    .arg(arg!(--qos_history <QOS_HISTORY> "History of samples setting (default: keep_last)"))
                    .arg(arg!(--qos_reliability <QOS_RELIABILITY> "Reliability setting (default: Auto match)"))
                    .arg(arg!(--qos_durability <QOS_DURABILITY> "Durability setting (default: Auto match)"))
                    .arg(arg!(--csv "Output fields separated by commas (for plotting)"))
                    .arg(arg!(--field <FIELD> "Echo selected field of a message"))
                    .arg(arg!(-f --full_length "Output all elements for arrays, bytes, and long strings"))
                    .arg(arg!(-l --truncate_length <TRUNCATE_LENGTH> "Length to truncate arrays/strings to (default: 128)"))
                    .arg(arg!(--no_arr "Don't print array fields"))
                    .arg(arg!(--no_str "Don't print string fields"))
                    .arg(arg!(--flow_style "Print collections in block style (not in csv)"))
                    .arg(arg!(--no_lost_messages "Don't report lost messages"))
                    .arg(arg!(--raw "Echo raw binary representation"))
                    .arg(arg!(--once "Print first message and exit"))
                )
                .subcommand(
                    Command::new("hz")
                    .about("Display publishing rate of topic")
                    .aliases(["h", "rate"])
                    .arg_required_else_help(true)
                    .arg(
                        arg!(<TOPIC_NAME> "Name of the ROS topic to mnitor (e.g. '/chatter')")
                        .required(true)
                    )  
                    .arg(arg!(-w --window <WINDOW> "Window size for rate calculation (default: 10000)"))
                    .arg(arg!(--filter <EXPR> "Filter messages by Python expression"))
                    .arg(arg!(--wall_time "Calculate rate using wall time (useful when clock is not published)"))
                    .arg(arg!(--spin_time <SPIN_TIME> "Spin time in seconds for discovery (if daemon not in use)"))
                    .arg(arg!(-s --use_sim_time "Enable ROS simulation time"))                    
                )
                .subcommand(
                    Command::new("info")
                    .about("Print information about a topic")
                    .aliases(["i", "show"])
                    .arg_required_else_help(true)
                    .arg(
                        arg!(<TOPIC_NAME> "Name of the ROS topic to get info (e.g. '/chatter')")
                        .required(true)
                    )
                    .arg(arg!(--spin_time <SPIN_TIME> "Spin time for discovery (if daemon not in use)"))
                    .arg(arg!(-s --use_sim_time "Enable ROS simulation time"))
                    .arg(arg!(--no_daemon "Don't spawn or use a running daemon"))
                    .arg(arg!(-v --verbose "Print detailed information about nodes, namespaces, topic types..."))                    
                )
                .subcommand(
                    Command::new("list")
                    .about("Output a list of available topics")
                    .aliases(["l", "ls"])
                    .arg_required_else_help(true)
                    .arg(arg!(--spin_time <SPIN_TIME> "Spin time for discovery (if daemon not in use)"))
                    .arg(arg!(-s --use_sim_time "Enable ROS simulation time"))
                    .arg(arg!(--no_daemon "Don't spawn or use a running daemon"))
                    .arg(arg!(-t --show_types "Additionally show the topic type"))
                    .arg(arg!(-c --count_topics "Only display the number of topics discovered"))
                    .arg(arg!(--include_hidden_topics "Consider hidden topics as well"))
                )
                .subcommand(
                    Command::new("pub")
                    .about("Publish a message to a topic")
                    .aliases(["p", "publish"])
                    .arg_required_else_help(true)
                    .arg(
                        arg!(<TOPIC_NAME> "Name of the ROS topic to publish to (e.g. '/chatter')")
                        .required(true)
                    )
                    .arg(
                        arg!(<MESSAGE_TYPE> "Type of the ROS message (e.g. 'std_msgs/msg/String')")
                        .required(true)
                    )
                    .arg(
                        arg!(<VALUES> "Values to fill the message with in YAML format (e.g. 'data: Hello World')")
                        .required(true)
                    )
                    .arg(arg!(-r --rate <RATE> "Publishing rate in Hz (default: 1)"))
                    .arg(arg!(-p --print <PRINT> "Print every N-th message (default: 1)"))
                    .arg(arg!(--once "Publish one message and exit"))
                    .arg(arg!(-t --times <TIMES> "Publish this many times and exit"))
                    .arg(arg!(-w --wait_matching_subscriptions <WAIT_MATCHING_SUBS> "Wait for specified matching subscriptions"))
                    .arg(arg!(--keep_alive <KEEP_ALIVE> "Keep node alive N sec after last message (default: 0.1)"))
                    .arg(arg!(-n --node_name <NODE_NAME> "Name of publishing node"))
                    .arg(arg!(--qos_profile <QOS_PROFILE> "QoS preset profile to publish"))
                    .arg(arg!(--qos_depth <QOS_DEPTH> "Queue size setting (overrides profile)"))
                    .arg(arg!(--qos_history <QOS_HISTORY> "History of samples setting (overrides profile, default: keep_last)"))
                    .arg(arg!(--qos_reliability <QOS_RELIABILITY> "QoS reliability setting (overrides profile, default: reliable)"))
                    .arg(arg!(--qos_durability <QOS_DURABILITY> "QoS durability setting (overrides profile, default: transient_local)"))
                    .arg(arg!(--spin_time <SPIN_TIME> "Spin time for discovery (if daemon not used)"))
                    .arg(arg!(-s --use_sim_time "Enable ROS sim time"))
                )
                .subcommand(
                    Command::new("kind")
                    .about("Print a topic's type/kind")
                    .aliases(["k", "type"])
                    .arg_required_else_help(true)
                    .arg(
                        arg!(<TOPIC_NAME> "Name of the ROS topic to get type (e.g. '/chatter')")
                        .required(true)
                    )
                    .arg(arg!(--spin_time <SPIN_TIME> "Spin time for discovery (if daemon not in use)"))
                    .arg(arg!(-s --use_sim_time "Enable ROS simulation time"))
                    .arg(arg!(--no_daemon "Don't spawn or use a running daemon"))
                )
                .subcommand(
                    Command::new("bw")
                    .about("Display bandwidth used by topic")
                    .aliases(["b", "bandwidth"])
                    .arg_required_else_help(true)
                    .arg(
                        arg!(<TOPIC_NAME> "Name of the ROS topic to get bandwidth (e.g. '/chatter')")
                        .required(true)
                    )
                    .arg(arg!(-w --window <WINDOW> "Window size for rate calculation (default: 10000)"))
                    .arg(arg!(--spin_time <SPIN_TIME> "Spin time for discovery (if daemon not used)"))
                    .arg(arg!(-s --use_sim_time "Enable ROS sim time"))
                )
                .subcommand(
                    Command::new("find")
                    .about("Output a list of available topics of a given type")
                    .aliases(["f", "lookup", "search"])
                    .arg_required_else_help(true)
                    .arg(
                        arg!(<TOPIC_TYPE> "Name of the ROS topic type to filter for (e.g. 'std_msg/msg/String')")
                        .required(true)
                    )
                    .arg(arg!(--spin_time <SPIN_TIME> "Spin time for discovery (if daemon not in use)"))
                    .arg(arg!(-s --use_sim_time "Enable ROS simulation time"))
                    .arg(arg!(--no_daemon "Don't spawn or use a running daemon"))
                    .arg(arg!(-c --count_topics "Only display the number of topics discovered"))
                    .arg(arg!(--include_hidden_topics "Consider hidden topics as well"))
                )
                .subcommand(
                    Command::new("delay")
                    .about("Display delay of topic from timestamp in header")
                    .aliases(["d", "latency"])
                    .arg_required_else_help(true)
                    .arg(
                        arg!(<TOPIC_NAME> "Name of the ROS topic to get delay (e.g. '/chatter')")
                        .required(true)
                    )
                    .arg(arg!(-w --window <WINDOW> "Window size for rate calculation (default: 10000)"))
                    .arg(arg!(--spin_time <SPIN_TIME> "Spin time for discovery (if daemon not used)"))
                    .arg(arg!(-s --use_sim_time "Enable ROS sim time"))
                )
        )
        .subcommand(
            Command::new("service")
                .about(aly("s", subs) + "Various service subcommands")
                .aliases(&["s", "ser"])
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
                .about(aly("p", subs) + "Various param subcommands")
                .aliases(&["p", "par"])
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
                .about(aly("n", subs) + "Various node subcommands")
                .aliases(&["n", "nod"])
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
                .about(aly("i", subs) + "Various interface subcommands\n")
                .aliases(&["i", "int"])
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
                .about(aly("r", subs) + "Run an executable")
                .aliases(&["r"])
                .subcommand_required(true)
                .arg_required_else_help(true)
        )
        .subcommand(
            Command::new("launch")
                .about(aly("l", subs) + "Launch a launch file\n")
                .aliases(&["l"])
                .subcommand_required(true)
                .arg_required_else_help(true)
        )

        .subcommand(
            Command::new("pkg")
                .about("Various package subcommands")
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
                .about("Various rosbag subcommands")
                .subcommand_required(true)
                .arg_required_else_help(true)
        )
}