use std::collections::HashMap;

pub fn uncommon_words(a: String, b: String) -> Vec<String> {
    let mut all_words = HashMap::new();
    let words = format!("{} {}", a, b);

    for s in words.split_whitespace() {
        let count = all_words.entry(s).or_insert(0);
        *count += 1;
    }

    all_words
        .into_iter()
        .filter(|(_, value)| *value == 1)
        .map(|(key, _)| key.to_string())
        .collect::<Vec<String>>()
}