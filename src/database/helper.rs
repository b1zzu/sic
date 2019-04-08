use std::fs::File;
use std::path::PathBuf;

use crate::utils::password::ask_password;

use super::card::Card;
use super::decrypt::decrypt;
use super::parse::parse;

pub fn open(database: PathBuf, password: Option<String>) -> Vec<Card> {
    let database = File::open(database).unwrap();

    let password = password.unwrap_or_else(ask_password);

    let database = decrypt(database, password.as_bytes());

    parse(database.as_slice())
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::path::Path;

    use super::*;

    #[test]
    fn test_parse() {
        let database = Path::new("./samples/SafeInCloud.db").to_path_buf();
        let password = "TheHarde5tPassw@ord!nT#3World".to_string();
        let database = open(database, Some(password));

        assert_eq!(database.len(), 5);
    }
}