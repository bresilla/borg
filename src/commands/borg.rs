use clap::ArgMatches;

pub fn handle(matches: ArgMatches) {
    match matches.subcommand() {
        Some(("action", action)) => {
            match action.subcommand() {
                Some(("list", _list)) => {
                    println!("list");
                }
                Some(("info", _info)) => {
                    println!("info");
                }
                Some(("goal", _goal)) => {
                    println!("goal");
                }
                _ => unreachable!("UNREACHABLE"),
            }
        }
        _ => unreachable!("UNREACHABLE"),
    };
}