use std::collections::HashMap;

pub fn word_pattern(pattern: String, r#str: String) -> bool {
    let pattern_vec: Vec<char> = pattern.chars().collect();
    let str_vec: Vec<&str> = r#str.split_whitespace().collect();

    if pattern_vec.len() != str_vec.len() {
        return false;
    }

    let mut letter_word_map: HashMap<char, &str> = HashMap::new();

    for e in pattern_vec.iter().zip(str_vec.iter()) {
        if letter_word_map.contains_key(e.0) {            
            let saved_word = letter_word_map.get(e.0).unwrap();
            if saved_word != e.1 {
                return false;
            }
        } else {
            if letter_word_map.values().any(|&word| &word == e.1) {
                return false;
            }
            letter_word_map.insert(*e.0, e.1);
        }
    }
    true
}