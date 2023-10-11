use clap::ArgMatches;
use std::process::Stdio;
use tokio::process::Command;
use tokio::io::AsyncReadExt;

async fn run_command(matches: ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let mut command = "ros2 topic hz".to_owned();

    let topic_name = matches.get_one::<String>("topic_name").unwrap();
    command.push_str(" ");
    command.push_str(&topic_name.to_string());

    if matches.get_one::<String>("window") != None {
        let window_value = matches.get_one::<String>("window").unwrap();
        command.push_str(" --window ");
        command.push_str(&window_value.to_string());
    }
    if matches.get_one::<String>("spin_time") != None {
        let spin_time_value = matches.get_one::<String>("spin_time").unwrap();
        command.push_str(" --spin-time ");
        command.push_str(&spin_time_value.to_string());
    }
    if matches.get_flag("use_sim_time") {
        command.push_str(" --use-sim-time");
    }
    if matches.get_one::<String>("filter") != None {
        let filter_value = matches.get_one::<String>("filter").unwrap();
        command.push_str(" --filter ");
        command.push_str(&filter_value.to_string());
    }
    if matches.get_flag("wall_time") {
        command.push_str(" --wall-time");
    }

    let mut cmd = Command::new("bash")
    .arg("-c")
    .arg(command)
    .stdout(Stdio::piped())
    .spawn()?;

    let stdout = cmd.stdout.take().unwrap();
    let mut reader = tokio::io::BufReader::new(stdout);

    let mut buffer = [0u8; 1024];
    loop {
        let n = reader.read(&mut buffer).await?;
        if n == 0 {
            break;
        }

        let output = String::from_utf8_lossy(&buffer[0..n]);
        print!("{}", output);
    }
    Ok(())
}

pub fn handle(matches: ArgMatches){
    let rt = tokio::runtime::Runtime::new().unwrap();
    let _ = rt.block_on(run_command(matches));
}