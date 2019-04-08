use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use xml::EventReader;
use xml::reader::XmlEvent::{CData, Characters, Comment, EndDocument, EndElement, ProcessingInstruction, StartDocument, StartElement, Whitespace};

use crate::database::decrypt::decrypt;
use crate::database::field::Field;
use crate::utils::password;

use super::card::Card;

pub struct Database {
    cards: Vec<Card>
}

impl Database {
    pub fn parse(source: impl Read) -> Self {
        let reader = EventReader::new(source);

        let mut cards = Vec::new();

        for event in reader {
            match event.unwrap() {
                StartElement { name, attributes, .. } => {
                    match name.local_name.as_str() {
                        "card" => {
                            cards.push(Card::parse(attributes).unwrap());
                        }
                        "field" => {
                            cards.last_mut().unwrap().add_field(Field::parse(attributes).unwrap());
                        }
                        "database" | "label" | "label_id" | "notes" | "ghost" | "custom_icon" => {
                            // ignore
                        }
                        _ => {
                            panic!("unhandled element: {:?}", name)
                        }
                    }
                }
                Characters(characters) => {
                    if let Some(card) = cards.last_mut() {
                        if let Some(filed) = card.get_last_field_mut() {
                            if filed.get_value() == None {
                                filed.set_value(characters);
                            }
                        }
                    }
                }
                StartDocument { .. }
                | EndDocument
                | ProcessingInstruction { .. }
                | Whitespace(_)
                | EndElement { .. }
                | Comment(_)
                | CData(_) => {
                    // ignore
                }
            }
        }

        // clean deleted cards and templates
        let cards = cards.into_iter().filter(|card| !card.is_deleted() && !card.is_template()).collect();

        Database { cards }
    }

    pub fn open(database: PathBuf, password: Option<String>) -> Self {
        let database = File::open(database).unwrap();

        let password = password.unwrap_or_else(password::ask_password);

        let database = decrypt(database, password.as_bytes());

        Self::parse(database.as_slice())
    }

    pub fn get_cards(&self) -> &Vec<Card> {
        &self.cards
    }
}


#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::path::Path;

    use crate::database::field;

    use super::*;

    #[test]
    fn test_database_parse() {
        let file = File::open("./samples/SafeInCloud.xml").unwrap();

        let database = Database::parse(file);
        let cards = database.get_cards();

        // templates are removed
        assert!(cards.iter().find(|card| card.get_title() == "Login/Password").is_none());

        // test exact number of cards
        assert_eq!(cards.len(), 5);

        // test fields of one card
        let facebook = cards.iter().find(|card| card.get_title() == "Facebook").unwrap();
        let facebook_login = facebook.get_field("Login").unwrap();
        let facebook_password = facebook.get_field("Password").unwrap();

        assert_eq!(facebook_login.get_value().unwrap(), "john555@gmail.com");
        assert_eq!(facebook_login.get_type(), field::Type::Login);
        assert_eq!(facebook_login.get_autofill().unwrap(), "username");

        assert_eq!(facebook_password.get_value().unwrap(), "early91*Fail*");
        assert_eq!(facebook_password.get_type(), field::Type::Password);
        assert_eq!(facebook_password.get_autofill().unwrap(), "current-password");
    }

    #[test]
    fn test_database_open() {
        let database = Path::new("./samples/SafeInCloud.db").to_path_buf();
        let password = "TheHarde5tPassw@ord!nT#3World".to_string();
        let database = Database::open(database, Some(password));

        assert_eq!(database.get_cards().len(), 5);
    }
}
