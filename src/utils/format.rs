type Table = Vec<Vec<(Option<String>, String)>>;

fn table(table: Table) -> String {
    "".to_string()
}

#[cfg(test)]
mod test {
    use regex::Regex;

    use super::*;

    #[test]
    fn test_table() {
        let t = vec![
            vec![(None, "1".to_string()), (None, "Facebook".to_string()), (Some("Login".to_string()), "fantastic@email.com".to_string())],
            vec![(None, "27".to_string()), (None, "GMail".to_string()), (Some("Password".to_string()), "*******".to_string())],
        ];

        let expected = "\
 1 | Facebook | Login:    fantastic@email.com
27 | GMail    | Password: *******            ";

        assert_eq!(table(t), expected);
    }
}