mod cli;
mod commands;
use std::env;

fn main() {
    let show_logo = if env::args().len() > 1 { false } else { true };
    let matches = cli::borg::cli(show_logo).get_matches();
    commands::borg::handle(matches);
}
