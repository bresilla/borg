use clap::ArgMatches;
use std::process::Stdio;
use tokio::process::Command;
use tokio::io::AsyncReadExt;

async fn run_command(matches: ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let mut command = "ros2 interface list".to_owned();

    if matches.get_flag("messages") {
        command.push_str(" --only-msgs");
    }
    if matches.get_flag("services") {
        command.push_str(" --only-srvs");
    }
    if matches.get_flag("actions") {
        command.push_str(" --only-actions");
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