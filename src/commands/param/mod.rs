pub mod get;
pub mod list;
pub mod set;
pub mod export;
pub mod remove;

use clap::ArgMatches;

pub fn handle(matches: ArgMatches){
    match matches.subcommand() {
        Some(("get", args)) => {
            get::handle(args.clone());
        }
        Some(("list", args)) => {
            list::handle(args.clone());
        }
        Some(("set", args)) => {
            set::handle(args.clone());
        }
        Some(("export", args)) => {
            export::handle(args.clone());
        }
        Some(("remove", args)) => {
            remove::handle(args.clone());
        }
        _ => unreachable!("UNREACHABLE"),
    }
}