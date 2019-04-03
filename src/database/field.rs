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
}

#[derive(Debug)]
pub struct Field {
    name: String,
    tipo: Type,
    autofill: String,
    value: Option<String>,
}

impl Field {
    pub fn new(name: String, tipo: Type, autofill: String) -> Field {
        Field { name, tipo, autofill, value: None }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_type(&self) -> Type {
        self.tipo
    }

    pub fn get_autofill(&self) -> &str {
        &self.autofill
    }

    pub fn get_value(&self) -> Option<&str> {
        match &self.value {
            Some(tipo) => Some(tipo),
            None => None,
        }
    }

    pub fn set_value(&mut self, value: String) {
        match self.value {
            None => self.value = Some(value),
            Some(_) => panic!("Field.value can be set only one time"),
        }
    }
}
