mod cli;
// use cli::derive;
// use clap::Parser;
use std::env;

fn main() {
    // let cli = derive::Cli::parse();
    let show_logo = if env::args().len() > 1 { false } else { true };
    let _matches = cli::borg::cli(show_logo).get_matches();

}
