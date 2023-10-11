pub mod list;
pub mod hz;

use clap::ArgMatches;

pub fn handle(matches: ArgMatches){
    match matches.subcommand() {
        Some(("list", args)) => {
            list::handle(args.clone());
        }
        Some(("hz", args)) => {
            hz::handle(args.clone());
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