use crate::utils::option;

use super::field::Field;

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