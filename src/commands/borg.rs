use clap::ArgMatches;

use crate::commands::*;


pub fn handle(matches: ArgMatches) {
    match matches.subcommand() {
        Some(("topic", submatch)) => {
            topic::handle(submatch.clone());
        }
        Some(("action", submatch)) => {
            action::handle(submatch.clone());
        }
        Some(("service", submatch)) => {
            service::handle(submatch.clone());
        }
        _ => unreachable!("UNREACHABLE"),
    };
}