use std::fs::File;
use std::path::PathBuf;

use crate::database::{decrypt, field, helper, parse};
use crate::utils::password;

pub struct Options {
    passwords: bool,
}

impl Options {
    pub fn new(passwords: bool) -> Self {
        Options { passwords }
    }
}

pub fn cards(database: PathBuf, options: Options) {
    let database = helper::open(database, None);

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