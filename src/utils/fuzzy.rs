pub trait Searchable {
    fn value(&self) -> &str;
}

struct S<T> {
    last: usize,
    points: usize,
    object: T,
}

impl<T> S<T> {
    fn new(object: T) -> Self {
        S { last: 0, points: 0, object }
    }
}

pub fn fuzzy<'a, T: Searchable>(stack: &'a [T], needle: &str) -> Vec<&'a T> {
    let mut r = vec![];
    for o in stack {
        let mut s = S::new(o);
        let mut v = o.value();
        for n in needle.chars() {
            match v.find(n) {
                Some(p) => {
                    s.points += if s.last + 1 == p {
                        p - s.last
                    } else {
                        p
                    };
                    s.last = p;
                }
                None => {
                    s.points = 0;
                    break;
                }
            }
        }
        if s.points != 0 {
            r.push(s);
        }
    }

    r.sort_unstable_by_key(|s| s.points);

    r.iter().map(|s| s.object).collect()
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
        let result = fuzzy(&["Facebook", "Twitter", "Baked"], "ak");
        assert_eq!(result, vec![&"Baked", &"Facebook"]);
    }

    #[test]
    fn test_fuzzy_same_match_different_position() {
        let result = fuzzy(&["Facebook", "Book", "other"], "book");
        assert_eq!(result, vec![&"book", &"Facebook"])
    }

    #[test]
    fn test_fuzzy_order_of_needle() {
        let result = fuzzy(&["Facebook"], "ko");
        assert_eq!(result.len(), 0)
    }
}