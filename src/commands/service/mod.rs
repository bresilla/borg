pub mod call;
pub mod find;
pub mod list;
pub mod kind;

use clap::ArgMatches;

pub fn handle(matches: ArgMatches){
    match matches.subcommand() {
        Some(("call", args)) => {
            call::handle(args.clone());
        }
        Some(("find", args)) => {
            find::handle(args.clone());
        }
        Some(("list", args)) => {
            list::handle(args.clone());
        }
        Some(("kind", args)) => {
            kind::handle(args.clone());
        }
        _ => unreachable!("UNREACHABLE"),
    }
}