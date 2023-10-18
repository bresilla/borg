pub mod list;
pub mod package;

use clap::ArgMatches;

pub fn handle(matches: ArgMatches){
    match matches.subcommand() {
        Some(("list", args)) => {
            list::handle(args.clone());
        }
        Some(("package", args)) => {
            package::handle(args.clone());
        }
        _ => unreachable!("UNREACHABLE"),
    }
}