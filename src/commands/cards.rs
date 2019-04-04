use std::path::PathBuf;

pub fn cards(database: PathBuf) {
    // TODO:
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::*;

    #[test]
    fn test_cards() {
        let database = Path::new("./samples/SafeInCloud.dh");
        cards(database.to_path_buf());
    }
}