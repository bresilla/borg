use std::process::Command;
use clap::ArgMatches;

pub fn handle(matches: ArgMatches){

    let mut command = "ros2 topic list".to_owned();

    if matches.get_flag("show_types") {
        command.push_str(" --show-types");
    }
    if matches.get_flag("count_topics") {
        command.push_str(" --count-topics");
    }
    if matches.get_flag("include_hidden_topics") {
        command.push_str(" --include-hidden-topics");
    }
    if matches.get_flag("use_sim_time") {
        command.push_str(" --use-sim-time");
    }
    if matches.get_flag("no_daemon") {
        command.push_str(" --no-daemon");
    }
    if matches.get_one::<String>("spin_time") != None {
        let spin_time_value = matches.get_one::<String>("spin_time").unwrap();
        command.push_str(" --spin-time ");
        command.push_str(&spin_time_value.to_string());
    }

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
