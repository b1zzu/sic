pub trait Searchable {
    fn value(&self) -> &str;
}

pub fn fuzzy<T: Searchable>(stack: &[T], needle: &str) -> Vec<T> {
    vec![]
}

#[cfg(test)]
mod test {
    use super::*;

    impl Searchable for &str {
        fn value(&self) -> &str {
            &self
        }
    }

    #[test]
    fn test_fuzzy() {
        fuzzy(&["one", "two"], "wo");
    }
}