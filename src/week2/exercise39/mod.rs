use std::collections::HashMap;

pub fn most_common(paragraph: String, banned: Vec<String>) -> String {
    let mut count_words = HashMap::new();

    paragraph.split_whitespace().for_each(|word| {
        let count = count_words.entry(valid_word(word)).or_insert(0);
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

fn valid_word(word: &str) -> String {
    word.chars()
        .filter(|c| c.is_alphanumeric())
        .collect::<String>()
        .to_lowercase()
}
