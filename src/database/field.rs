use xml::attribute::OwnedAttribute;

use crate::utils::option;

use super::result::Result;
use super::result::to_result;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Type {
    Login,
    Password,
    Number,
    Text,
    Expiry,
    Pin,
    Phone,
    Website,
    Date,
    Email,
    Application,
    Secret,
}

impl Type {
    fn parse(tipo: String) -> Self {
        match tipo.as_str() {
            "login" => Type::Login,
            "password" => Type::Password,
            "pin" => Type::Pin,
            "number" => Type::Number,
            "text" => Type::Text,
            "expiry" => Type::Expiry,
            "phone" => Type::Phone,
            "website" => Type::Website,
            "date" => Type::Date,
            "email" => Type::Email,
            "application" => Type::Application,
            "secret" => Type::Secret,
            _ => panic!("unhandled field type: {}", tipo)
        }
    }
}

#[derive(Debug)]
pub struct Field {
    name: String,
    tipo: Type,
    autofill: Option<String>,
    value: Option<String>,
}

impl Field {
    pub fn new(name: String, tipo: Type, autofill: Option<String>) -> Self {
        Field { name, tipo, autofill, value: None }
    }

    pub fn parse(attributes: Vec<OwnedAttribute>) -> Result<Self> {
        let mut name = None;
        let mut tipo = None;
        let mut autofill = None;

        for attribute in attributes {
            match attribute.name.local_name.as_str() {
                "name" => {
                    name = Some(attribute.value);
                }
                "type" => {
                    tipo = Some(Type::parse(attribute.value));
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

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_type(&self) -> Type {
        self.tipo
    }

    pub fn get_autofill(&self) -> Option<&str> {
        option::as_str(&self.autofill)
    }

    pub fn get_value(&self) -> Option<&str> {
        option::as_str(&self.value)
    }

    pub fn set_value(&mut self, value: String) {
        match self.value {
            None => self.value = Some(value),
            Some(_) => panic!("Field.value can be set only one time"),
        }
    }
}
