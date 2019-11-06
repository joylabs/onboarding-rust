use std::collections::HashMap;
pub fn most_common_word(paragraph: String, banned: Vec<String>) -> String {
    let paragraph = paragraph
        .to_lowercase()
        .replace(&['!', '?', '\'', ',', ';', '.'][..], " ");
    let mut words_count: HashMap<String, i32> = HashMap::new();

    paragraph
        .split_ascii_whitespace()
        .filter(|word| !banned.contains(&(*word).to_string()))
        .for_each(|word| {
            *words_count
                .entry(word.to_string().to_lowercase())
                .or_insert(0) += 1;
        });

    let most_common = words_count.iter().max_by_key(|(_, v)| *v).unwrap();
    most_common.0.to_string()
}
