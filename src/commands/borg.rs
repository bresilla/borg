use clap::ArgMatches;

use crate::commands::*;


pub fn handle(matches: ArgMatches) {
    match matches.subcommand() {
        Some(("action", submatch)) => {
           action::main::handle(submatch.clone());
        }
        _ => unreachable!("UNREACHABLE"),
    };
}