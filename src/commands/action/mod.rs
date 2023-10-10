pub mod list;

use clap::ArgMatches;

pub fn handle(matches: ArgMatches){
    match matches.subcommand() {
        Some(("list", list)) => {
            list::handle(list.clone());
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