use std::process::Command;
use clap::ArgMatches;

pub fn handle(_matches: ArgMatches){
    let output = Command::new("bash")
        .arg("-c")
        .arg("ros2 action list")
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
