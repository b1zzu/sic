use std::fs::File;
use std::path::PathBuf;

use crate::database::{decrypt, field, parse};

pub struct Options {
    pub passwords: bool
}

pub fn cards(database: PathBuf, options: Options) {
    let database = File::open(database).unwrap();

    let password = rpassword::read_password_from_tty(Some("Password: ")).unwrap();

    let database = decrypt::decrypt(database, password.as_bytes());

    let database = parse::parse(database.as_slice());

    println!();
    for card in database {
        print!("{:12} | {:24}", card.get_id(), card.get_title());

        let mut i = 0;
        for field in card.get_fields() {
            i += 1;
            if i > 3 {
                break;
            }

            let blank = String::new();

            let value = match field.get_type() {
                field::Type::Password | field::Type::Secret | field::Type::Pin => {
                    if options.passwords {
                        field.get_value().unwrap_or(&blank)
                    } else {
                        "******"
                    }
                }
                _ => field.get_value().unwrap_or(&blank)
            };

            print!(" | {:12}: {:32}", field.get_name(), value);
        }

        println!();
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