use std::collections::HashSet;

pub fn longest_word(mut words: Vec<String>) -> String {
    let mut longest_first_word = String::new();
    let mut hash_set_words = HashSet::new();

    words.sort();

    for word in words {

        if word.len() == 1 {
            hash_set_words.insert(word.clone());
            if word.len() > longest_first_word.len() {
                longest_first_word = word;
            }
        } else {
            let one_less_char = word.len() - 1;

            if hash_set_words.contains(&word[..one_less_char]) {
                hash_set_words.insert(word.clone());
                if word.len() > longest_first_word.len() {
                    longest_first_word = word;
                }
            }
        }

    }
    longest_first_word
}
