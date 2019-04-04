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

#[derive(Debug)]
pub struct Field {
    name: String,
    tipo: Type,
    autofill: Option<String>,
    value: Option<String>,
}

impl Field {
    pub fn new(name: String, tipo: Type, autofill: Option<String>) -> Field {
        Field { name, tipo, autofill, value: None }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_type(&self) -> Type {
        self.tipo
    }

    pub fn get_autofill(&self) -> Option<&String> {
        self.autofill.as_ref()
    }

    pub fn get_value(&self) -> Option<&String> {
        self.value.as_ref()
    }

    pub fn set_value(&mut self, value: String) {
        match self.value {
            None => self.value = Some(value),
            Some(_) => panic!("Field.value can be set only one time"),
        }
    }
}
