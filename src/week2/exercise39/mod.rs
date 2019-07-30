use std::collections::HashMap;

pub fn most_common_not_banned(paragraph: String, banned: Vec<String>) -> String {
    let word_counts = paragraph
        .split(|c: char| !c.is_alphabetic())
        .filter(|w| !w.is_empty())
        .fold(HashMap::new(), |mut acc, x| {
            *acc.entry((*x).to_string().clone().to_lowercase())
                .or_insert(0) += 1;
            acc
        });

    word_counts
        .iter()
        .filter(|(key, _)| !banned.contains(*key))
        .map(|(key, value)| (value, key))
        .max()
        .map(|(_, key)| key)
        .unwrap()
        .to_string()
}

