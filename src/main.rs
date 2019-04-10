use std::path::Path;

use clap::{App, Arg, SubCommand};

use commands::cards;

mod commands;
mod database;
mod utils;

fn main() {
    let matches = App::new("sic")
        .version("0.1.2")
        .about("Command Line Tool for SafeInCloud")
        .arg(
            Arg::with_name("database")
                .long("database")
                .short("b")
                .value_name("FILE")
                .help("Set a custom path to the SafeInCloud db")
                .takes_value(true),
        )
        .subcommand(
            SubCommand::with_name("cards").about("Print all cards").arg(
                Arg::with_name("passwords")
                    .long("passwords")
                    .short("p")
                    .help("Print passwords, pins and secrets"),
            ),
        )
        .get_matches();

    // Get the path of the SafeInCloud database from the command line or from the default path
    let database = match matches.value_of("database") {
        Some(database) => Path::new(database).to_path_buf(),
        None => dirs::home_dir()
            .unwrap_or_default()
            .join("./.SafeInCloud.db"),
    };

    if let Some(matches) = matches.subcommand_matches("cards") {
        cards::cards(
            database,
            cards::Options::new(matches.is_present("passwords")),
        );
    }
}
