use core::result;

#[derive(Debug)]
pub struct Error {
    error: String,
}

impl Error {
    pub fn new(error: String) -> Error {
        Error { error }
    }
}

pub type Result<T> = result::Result<T, Error>;

pub fn to_result<T>(value: Option<T>, error: &str) -> Result<T> {
    match value {
        Some(value) => Ok(value),
        None => Err(Error::new(error.to_string())),
    }
}
