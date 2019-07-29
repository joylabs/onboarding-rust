use std::collections::HashMap;

pub fn most_common_not_banned(paragraph: String, banned: Vec<String>) -> String {
    let mut count_words = HashMap::new();

    paragraph
        .to_lowercase()
        .chars()
        .map(|ch| if ch.is_alphabetic() { ch } else { ' ' })
        .collect::<String>()
        .split_whitespace()
        .for_each(|word| {
            let count = count_words.entry(word.to_string()).or_insert(0);
            *count += 1;
        });

    count_words
        .iter()
        .filter(|(key, _)| !banned.contains(*key))
        .map(|(key, value)| (value, key))
        .max()
        .map(|(_, key)| key)
        .unwrap()
        .to_string()
}

