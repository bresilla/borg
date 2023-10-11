use clap::ArgMatches;

use crate::commands::*;


pub fn handle(matches: ArgMatches) {
    match matches.subcommand() {
        Some(("topic", submatch)) => {
            topic::handle(submatch.clone());
        }
        _ => unreachable!("UNREACHABLE"),
    };
}