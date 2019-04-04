use std::fs::File;
use std::path::PathBuf;

use crate::database::{decrypt, field, parse};

pub fn cards(database: PathBuf) {
    let database = File::open(database).unwrap();

    let password = rpassword::read_password_from_tty(Some("Password: ")).unwrap();

    let database = decrypt::decrypt(database, password.as_bytes());

    let database = parse::parse(database.as_slice());

    println!();
    for card in database {
        println!("==");
        println!("- Title: {}", card.get_title());
        println!("- Id:    {}", card.get_id());
        for field in card.get_fields() {
            match field.get_type() {
                field::Type::Password => println!("{}: ******", field.get_name()),
                _ => println!("{}: {}", field.get_name(), field.get_value().unwrap_or(&String::new()))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::*;

    #[test]
    fn test_cards() {
        // TODO: Use mock
//        let database = Path::new("./samples/SafeInCloud.db");
//        cards(database.to_path_buf());
    }
}