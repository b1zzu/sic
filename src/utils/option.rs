pub fn as_str(option: &Option<String>) -> Option<&str> {
    match option {
        Some(s) => Some(s.as_str()),
        None => None,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_as_str() {
        assert_eq!(as_str(&None), None);
        assert_eq!(as_str(&Some(String::from("test"))), Some("test"));
    }
}