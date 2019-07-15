use std::collections::HashMap;

pub fn word_pattern(pattern: String, str_s: String) -> bool {

    validate_pattern(pattern.chars().collect())
        == validate_pattern(str_s.split_whitespace().collect())
}

fn validate_pattern<T>(input: Vec<T>) -> String
where
    T: std::hash::Hash + std::cmp::Eq,
{
    let mut map = HashMap::new();
    input
        .iter()
        .enumerate()
        .map(|(i, ch)| map.entry(ch).or_insert(i).to_string())
        .collect::<String>()

}

