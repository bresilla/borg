mod cli;

use cli::derive;
use clap::Parser;
use cli::builder;

fn main() {
    // let cli = derive::Cli::parse();
    let cli2 = builder::cli().get_matches();

}
