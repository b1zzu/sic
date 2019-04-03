use clap::{App, SubCommand};

use crate::commands::cards::cards;

mod database;
mod utils;
mod commands;

fn main() {
    let matches = App::new("sic")
        .version("1.0")
        .about("Command Line Tool for SafeInCloud")
        .subcommand(SubCommand::with_name("cards")
            .about("print all cards"))
        .get_matches();

    if let Some(_) = matches.subcommand_matches("cards") {
        cards();
    }
}
