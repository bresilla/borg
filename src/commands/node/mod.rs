pub mod list;
pub mod info;

use clap::ArgMatches;

pub fn handle(matches: ArgMatches){
    match matches.subcommand() {
        Some(("info", args)) => {
            info::handle(args.clone());
        }
        Some(("list", args)) => {
            list::handle(args.clone());
        }
        _ => unreachable!("UNREACHABLE"),
    }
}