use std::fs::File;
use std::path::PathBuf;

use crate::database::{decrypt, field};
use crate::database::card::Card;
use crate::database::database::Database;
use crate::utils::{format, password};

pub struct Options {
    passwords: bool,
}

impl Options {
    pub fn new(passwords: bool) -> Self {
        Options { passwords }
    }
}

pub fn cards(database: PathBuf, options: Options) {
    let database = Database::open(database, None);
    println!("{}", format::table(database.to_table(options.passwords)))
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::*;

    #[test]
    fn test_cards() {}
}