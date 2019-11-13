use std::collections::HashSet;
pub fn longest_word(words: Vec<String>) -> String {
    let words2 = words.clone();
    let word_set: HashSet<String> = words2.into_iter().collect();
    let mut result = "".to_string();
    words.iter().for_each(|word| {
        let mut build_word = word.clone();
        while word_set.contains(&build_word) {
            build_word.pop();
        }

        if build_word.is_empty()
            && (word.len() > result.len()
                || (word.len() == result.len() && word.as_bytes() < result.as_bytes()))
        {
            result = word.to_string();
        }
    });
    result
}
