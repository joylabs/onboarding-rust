use std::collections::HashSet;

pub fn longest_word(words: Vec<String>) -> String {
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