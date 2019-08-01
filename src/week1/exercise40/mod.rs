use std::collections::BTreeMap;

pub fn most_common_word(paragraph: String, banned: Vec<String>) -> String {
    let mut words_map: BTreeMap<String, i32> = BTreeMap::new();

    paragraph
        .to_lowercase()
        .chars()
        .map(|c| if c.is_ascii_alphabetic() { c } else { ' ' })
        .collect::<String>()
        .split_whitespace()
        .filter(|word| !banned.contains(&word.to_string()))
        .for_each(|word| {
            let count = words_map.entry(word.to_string()).or_insert(0);
            *count += 1;
        });

    let mut max = 0;
    let mut max_word = "".to_string();
    words_map.iter().for_each(|(word, value)| {
        if *value > max {
            max = *value;
            max_word = word.clone();
        }
    });

    max_word
}