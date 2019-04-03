use std::io::Read;
use std::str::FromStr;

use xml::attribute::OwnedAttribute;
use xml::EventReader;
use xml::reader::XmlEvent::{CData, Characters, Comment, EndDocument, EndElement, ProcessingInstruction, StartDocument, StartElement, Whitespace};

use crate::database::card::Card;

use super::field::Field;

fn parse_field(attributes: Vec<OwnedAttribute>) -> Field {
    let mut name = None;
    let mut tipo = None;
    let mut autofill = None;

    for attribute in attributes {
        match attribute.name.local_name.as_str() {
            "name" => {
                name = Some(attribute.value);
            }
            "type" => {
                tipo = Some(attribute.value);
            }
            "autofill" => {
                autofill = Some(attribute.value);
            }
            "score" | "hash" => {
                // ignore
            }
            _ => {
                panic!("unhandled attribute: {:?}", attribute);
            }
        }
    }

    Field::new(name.unwrap(), tipo.unwrap(), autofill.unwrap())
}

fn parse_card(attributes: Vec<OwnedAttribute>) -> Card {
    let mut title = None;
    let mut id = None;
    let mut template = false;
    let mut deleted = false;
    let mut tipo = None;

    for attribute in attributes {
        match attribute.name.local_name.as_str() {
            "title" => {
                title = Some(attribute.value);
            }
            "id" => {
                id = Some(u32::from_str(attribute.value.as_str()).unwrap());
            }
            "template" => {
                template = bool::from_str(attribute.value.as_str()).unwrap();
            }
            "deleted" => {
                deleted = bool::from_str(attribute.value.as_str()).unwrap();
            }
            "type" => {
                tipo = Some(attribute.value);
            }
            "symbol" | "color" | "time_stamp" => {
                // ignore
            }
            _ => {
                panic!("unhandled attribute: {:?}", attribute)
            }
        }
    }

    Card::new(title.unwrap(), id.unwrap(), template, deleted, tipo)
}

enum Element {
    None,
    Field(Field),
    Card(Card),
}

fn parse(source: impl Read) -> Vec<Card> {
    let reader = EventReader::new(source);

    let mut cards = Vec::new();

    for event in reader {
        match event.unwrap() {
            StartElement { name, attributes, .. } => {
                match name.local_name.as_str() {
                    "card" => {
                        cards.push(parse_card(attributes));
                    }
                    "field" => {
                        cards.last_mut().unwrap().add_field(parse_field(attributes));
                    }
                    "database" | "label" | "label_id" | "notes" => {
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
    cards.into_iter().filter(|card| !card.is_deleted() && !card.is_template()).collect()
}

#[cfg(test)]
mod tests {
    use std::fs::File;

    use super::*;

    #[test]
    fn test_parse() {
        let file = File::open("./samples/SafeInCloud.xml").unwrap();

        let cards = parse(file);

        // templates are removed
        assert!(cards.iter().find(|card| card.get_title() == "Login/Password").is_none());

        // test exact number of cards
        assert_eq!(cards.len(), 5);

        // test fields of one card
        let facebook = cards.iter().find(|card| card.get_title() == "Facebook").unwrap();
        let facebook_login = facebook.get_field("Login").unwrap();
        let facebook_password = facebook.get_field("Password").unwrap();

        assert_eq!(facebook_login.get_value().unwrap(), "john555@gmail.com");
        assert_eq!(facebook_login.get_type(), "login");
        assert_eq!(facebook_login.get_autofill(), "username");

        assert_eq!(facebook_password.get_value().unwrap(), "early91*Fail*");
        assert_eq!(facebook_password.get_type(), "password");
        assert_eq!(facebook_password.get_autofill(), "current-password");
    }
}