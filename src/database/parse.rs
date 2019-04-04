use std::io::Read;
use std::result;
use std::str::FromStr;

use xml::attribute::OwnedAttribute;
use xml::EventReader;
use xml::reader::XmlEvent::{CData, Characters, Comment, EndDocument, EndElement, ProcessingInstruction, StartDocument, StartElement, Whitespace};

use crate::database::card::Card;

use super::field;
use super::field::Field;

#[derive(Debug)]
struct Error {
    error: String,
}

impl Error {
    fn new(error: String) -> Error {
        Error { error }
    }
}

type Result<T> = result::Result<T, Error>;

fn to_result<T>(value: Option<T>, error: &str) -> Result<T> {
    match value {
        Some(value) => Ok(value),
        None => Err(Error::new(error.to_string())),
    }
}

fn parse_field_type(tipo: String) -> field::Type {
    match tipo.as_str() {
        "login" => field::Type::Login,
        "password" => field::Type::Password,
        "pin" => field::Type::Pin,
        "number" => field::Type::Number,
        "text" => field::Type::Text,
        "expiry" => field::Type::Expiry,
        "phone" => field::Type::Phone,
        "website" => field::Type::Website,
        "date" => field::Type::Date,
        "email" => field::Type::Email,
        "application" => field::Type::Application,
        "secret" => field::Type::Secret,
        _ => panic!("unhandled field type: {}", tipo)
    }
}

fn parse_field(attributes: Vec<OwnedAttribute>) -> Result<Field> {
    let mut name = None;
    let mut tipo = None;
    let mut autofill = None;

    for attribute in attributes {
        match attribute.name.local_name.as_str() {
            "name" => {
                name = Some(attribute.value);
            }
            "type" => {
                tipo = Some(parse_field_type(attribute.value));
            }
            "autofill" => {
                autofill = Some(attribute.value);
            }
            "score" | "hash" | "history" => {
                // ignore
            }
            _ => {
                panic!("unhandled attribute in field: {:?}", attribute);
            }
        }
    }

    let name = to_result(name, "name in field is None")?;
    let tipo = to_result(tipo, "type in field is None")?;

    Ok(Field::new(name, tipo, autofill))
}

fn parse_card(attributes: Vec<OwnedAttribute>) -> Result<Card> {
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
            "symbol" | "color" | "time_stamp" | "website_icon" | "star" => {
                // ignore
            }
            _ => {
                panic!("unhandled attribute in card: {:?}", attribute)
            }
        }
    }

    let title = to_result(title, "title in card is None")?;
    let id = to_result(id, "id in card is None")?;

    Ok(Card::new(title, id, template, deleted, tipo))
}


pub fn parse(source: impl Read) -> Vec<Card> {
    let reader = EventReader::new(source);

    let mut cards = Vec::new();

    for event in reader {
        match event.unwrap() {
            StartElement { name, attributes, .. } => {
                match name.local_name.as_str() {
                    "card" => {
                        cards.push(parse_card(attributes).unwrap());
                    }
                    "field" => {
                        cards.last_mut().unwrap().add_field(parse_field(attributes).unwrap());
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
        assert_eq!(facebook_login.get_type(), field::Type::Login);
        assert_eq!(facebook_login.get_autofill().unwrap(), "username");

        assert_eq!(facebook_password.get_value().unwrap(), "early91*Fail*");
        assert_eq!(facebook_password.get_type(), field::Type::Password);
        assert_eq!(facebook_password.get_autofill().unwrap(), "current-password");
    }
}