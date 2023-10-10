use std::process::Command;
use clap::ArgMatches;

pub fn handle(matches: ArgMatches){

    let show_types = matches.get_flag("show_types");
    let count_topics = matches.get_flag("count_topics");
    let include_hidden_topics = matches.get_flag("include_hidden_topics");
    let use_sim_time = matches.get_flag("use_sim_time");
    let no_daemon = matches.get_flag("no_daemon");
    // let spin_time = matches.get_value_of("spin_time");

    let command = "ros2 topic list".to_owned() + 
        if show_types { " --show-types" } else { "" } +
        if count_topics { " --count-topics" } else { "" } +
        if include_hidden_topics { " --include-hidden-topics" } else { "" } +
        if use_sim_time { " --use-sim-time" } else { "" } +
        if no_daemon { " --no-daemon" } else { "" } +
        // if spin_time { " --spin-time" } else { "" } +
        "";

    let output = Command::new("bash")
        .arg("-c")
        .arg(command)
        .output()
        .expect("Failed to execute command");

    if !output.stdout.is_empty() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        println!("{}", stdout);
    }

    if !output.stderr.is_empty() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!("{}", stderr);
    }
}
