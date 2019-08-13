use std::collections::HashSet;

pub fn longest_word(mut words: Vec<String>) -> String {
    let mut longest_first_word = String::new();
    let mut hash_set_words = HashSet::new();

    words.sort();

    for mut word in words {
        if word.len() == 1 {
            hash_set_words.insert(word.clone());
        }
        let character = word.pop();

        if hash_set_words.contains(&word) {
            word.push(character.unwrap());
            hash_set_words.insert(word.clone());
            if word.len() > longest_first_word.len() {
                longest_first_word = word;
            }
        }
    }
    longest_first_word
}


pub fn longest_word_2(words: Vec<String>) -> String {
    let mut set = HashSet::new();
    let mut words_2 = words.clone();
    words_2.sort();

    let mut largest_word = "".to_string();

    for word in words_2 {
        let mut check_word = word.clone();
        check_word.pop();
        if word.len() == 1 || set.contains(&check_word) {
            set.insert(word.clone());
            if word.len() > largest_word.len() {
                largest_word = word;
            }
        }
    }
    largest_word
}
