pub mod list;
pub mod hz;
pub mod echo;

use clap::ArgMatches;

pub fn handle(matches: ArgMatches){
    match matches.subcommand() {
        Some(("list", args)) => {
            list::handle(args.clone());
        }
        Some(("hz", args)) => {
            hz::handle(args.clone());
        }
        Some(("echo", args)) => {
            echo::handle(args.clone());
        }
        _ => unreachable!("UNREACHABLE"),
    }
}