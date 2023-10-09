mod cli;
// use cli::derive;
// use clap::Parser;
use cli::builder;
use std::env;

fn main() {
    // let cli = derive::Cli::parse();
    let sub_commands = if env::args().len() > 1 { false } else { true };
    let _matches = builder::cli(sub_commands, true).get_matches();

}
