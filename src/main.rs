use std::path::{Path, PathBuf};

use clap::{App, Arg, SubCommand};

use commands::cards::cards;

use crate::commands::cards::Options;

mod database;
mod utils;
mod commands;

fn main() {
    let matches = App::new("sic")
        .version("1.0")
        .about("Command Line Tool for SafeInCloud")
        .arg(Arg::with_name("db")
            .long("db")
            .value_name("FILE")
            .help("Set a custom path to the SafeInCloud db")
            .takes_value(true))
        .subcommand(SubCommand::with_name("cards")
            .about("Print all cards")
            .arg(Arg::with_name("passwords")
                .long("passwords")
                .short("p")
                .help("Print passwords, pins and secrets")))
        .get_matches();

    let database = match matches.value_of("db") {
        Some(database) => Path::new(database).to_path_buf(),
        None => dirs::home_dir().unwrap_or(PathBuf::new()).join("./.SafeInCloud.db"),
    };

    if let Some(matches) = matches.subcommand_matches("cards") {
        cards(database, Options { passwords: matches.is_present("passwords") });
    }
}
