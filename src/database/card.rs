use std::str::FromStr;

use xml::attribute::OwnedAttribute;

use crate::database::result::to_result;
use crate::utils::option;

use super::field::Field;
use super::result::Result;

#[derive(Debug)]
pub struct Card {
    title: String,
    id: u32,
    template: bool,
    deleted: bool,
    tipo: Option<String>,
    fields: Vec<Field>,
}

impl Card {
    pub fn new(title: String, id: u32, template: bool, deleted: bool, tipo: Option<String>) -> Card {
        Card { title, id, template, deleted, tipo, fields: Vec::new() }
    }

    pub fn parse(attributes: Vec<OwnedAttribute>) -> Result<Card> {
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

    pub fn get_title(&self) -> &str {
        &self.title
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }

    pub fn is_template(&self) -> bool {
        self.template
    }

    pub fn is_deleted(&self) -> bool {
        self.deleted
    }

    pub fn get_type(&self) -> Option<&str> {
        option::as_str(&self.tipo)
    }

    pub fn get_fields(&self) -> &[Field] {
        self.fields.as_slice()
    }

    pub fn get_field(&self, name: &str) -> Option<&Field> {
        self.fields.iter().find(|field| field.get_name() == name)
    }

    pub fn get_last_field_mut(&mut self) -> Option<&mut Field> {
        self.fields.last_mut()
    }

    pub fn add_field(&mut self, field: Field) {
        self.fields.push(field);
    }
}