pub mod get;
pub mod list;

use clap::ArgMatches;

pub fn handle(matches: ArgMatches){
    match matches.subcommand() {
        Some(("get", args)) => {
            get::handle(args.clone());
        }
        Some(("list", args)) => {
            list::handle(args.clone());
        }
        _ => unreachable!("UNREACHABLE"),
    }
}