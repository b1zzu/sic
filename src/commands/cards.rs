use std::path::PathBuf;

use crate::database::database::Database;
use crate::utils::format;

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
    #[test]
    fn test_cards() {}
}
