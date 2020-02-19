use std::collections::HashMap;

pub fn uncommon_sentences_words(input_a: String, input_b: String) -> Vec<String> {
    let mut words_map = HashMap::new();
    let strings_togheter = format!("{} {}", input_a, input_b).to_string();

    strings_togheter.split_whitespace().for_each(|w| {
        let counter = words_map.entry(w).or_insert(0);
        *counter += 1
    });

    let mut result: Vec<String> = words_map
        .iter()
        .filter(|(_, w)| **w == 1)
        .map(|(w, _)| w.to_string())
        .collect();

    result.sort();

    result
}
