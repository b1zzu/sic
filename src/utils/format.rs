type Table = Vec<Vec<(Option<String>, String)>>;

pub fn table(table: Table) -> String {
    let mut measurements: Vec<(usize, usize)> = Vec::new();
    for row in &table {
        for (i, (left, right)) in row.iter().enumerate() {
            let left = match left {
                Some(left) => left.len() + 2,
                None => 0
            };
            let right = right.len();
            let measurement = measurements.get(i);
            match measurement {
                Some((left_m, right_m)) => {
                    let left = if left > *left_m { left } else { *left_m };
                    let right = if right > *right_m { right } else { *right_m };
                    measurements[i] = (left, right);
                }
                None => {
                    measurements.push((left, right));
                }
            }
        }
    }
    let measurements = measurements;

    let mut rows = Vec::new();
    for row in table {
        let mut columns = Vec::new();
        for (i, (left, right)) in row.iter().enumerate() {
            let (left_m, right_m) = measurements[i];
            let left = match left {
                Some(left) => format!("{}: ", left),
                None => String::new(),
            };
            columns.push(format!("{0:1$}{2:3$}", left, left_m, right, right_m));
        }
        rows.push(columns.join(" | "))
    }

    rows.join("\n")
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
1  | Facebook | Login:    fantastic@email.com
27 | GMail    | Password: *******            ";

        assert_eq!(table(t), expected);
    }
}