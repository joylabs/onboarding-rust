use std::collections::HashMap;

pub fn uncommon_words(a: String, b: String) -> Vec<String> {
    let string = a + " " + &b;
    let mut map = HashMap::new();

    for word in string.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    map.iter()
        .filter(|(_, value)| **value == 1)
        .map(|(key, _)| key.to_string())
        .collect()
}

