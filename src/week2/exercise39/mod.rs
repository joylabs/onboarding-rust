use std::collections::HashMap;

pub fn most_common(paragraph: String, banned: Vec<String>) -> String {
    let mut count_words = HashMap::new();
    let mut word: String = String::from("");

    for i in paragraph.chars() {
        if i.is_alphabetic() {
            word.push(i);
        } else {
            if !word.is_empty() {
                let count = count_words.entry(word.clone().to_lowercase()).or_insert(0);
                *count += 1;
            }
            word.clear();
        }
    }
    count_words.insert(word, 1);

    count_words
        .iter()
        .filter(|(key, _)| !banned.contains(*key))
        .map(|(key, value)| (value, key))
        .max()
        .map(|(_, key)| key)
        .unwrap()
        .to_string()
}

