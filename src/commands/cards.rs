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

fn to_table(cards: &Vec<Card>, options: Options) -> format::Table {
    let mut table = Vec::new();

    for card in cards {
        let mut row = Vec::new();

        row.push((None, card.get_id().to_string()));
        row.push((None, card.get_title().to_string()));

        let mut i = 0;
        for field in card.get_fields() {
            i += 1;
            if i > 3 {
                break;
            }

            let mask = "******";

            let value = match field.get_type() {
                field::Type::Password | field::Type::Secret | field::Type::Pin => {
                    if options.passwords {
                        field.get_value()
                    } else {
                        Some("******")
                    }
                }
                _ => field.get_value()
            };
            let value = value.unwrap_or("").to_string();

            let name = field.get_name().to_string().clone();

            row.push((Some(name), value));
        }

        table.push(row);
    }

    table
}

pub fn cards(database: PathBuf, options: Options) {
    let database = Database::open(database, None);
    let table = to_table(database.get_cards(), options);
    println!("{}", format::table(table))
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::*;

    #[test]
    fn test_cards() {}
}