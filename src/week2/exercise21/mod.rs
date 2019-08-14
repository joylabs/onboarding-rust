use std::collections::HashSet;

pub fn longest_word(mut words: Vec<String>) -> String {
    let mut longest_first_word = String::new();
    let mut hash_set_words = HashSet::new();

    words.sort();

    for mut word in words {
        let character;
        if word.len() == 1 {
            hash_set_words.insert(word.clone());
            if word.len() > longest_first_word.len() {
                longest_first_word = word;
            }
        } else {
            character = word.pop().unwrap();

            if hash_set_words.contains(&word) {
                word.push(character);
                hash_set_words.insert(word.clone());
                if word.len() > longest_first_word.len() {
                    longest_first_word = word;
                }
            }
        }

    }
    longest_first_word
}
