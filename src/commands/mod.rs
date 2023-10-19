pub mod action;
pub mod topic;
pub mod service;
pub mod param;
pub mod node;
pub mod interface;
pub mod frame;

pub mod run;
pub mod launch;
pub mod work;

pub mod bag;
pub mod daemon;
pub mod middleware;

use clap::ArgMatches;

pub fn handle(matches: ArgMatches) {
    match matches.subcommand() {
        Some(("action", submatch)) => {
            action::handle(submatch.clone());
        }
        Some(("topic", submatch)) => {
            topic::handle(submatch.clone());
        }
        Some(("service", submatch)) => {
            service::handle(submatch.clone());
        }
        Some(("param", submatch)) => {
            param::handle(submatch.clone());
        }
        Some(("node", submatch)) => {
            node::handle(submatch.clone());
        }
        Some(("interface", submatch)) => {
            interface::handle(submatch.clone());
        }
        _ => unreachable!("UNREACHABLE"),
    };
}