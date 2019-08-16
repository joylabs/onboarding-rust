use std::collections::HashSet;

pub fn unique_morse(words: Vec<String>) -> i32 {
    let mut unique_morse_words: HashSet<String>;
    unique_morse_words = words.into_iter().map(word_to_morse).collect();
    unique_morse_words.len() as i32
}

fn word_to_morse(word: String) -> String {
    let morse_dictionary: Vec<&str> = vec![
        ".-", "-...", "-.-.", "-..", ".", "..-.", "--.", "....", "..", ".---", "-.-", ".-..", "--",
        "-.", "---", ".--.", "--.-", ".-.", "...", "-", "..-", "...-", ".--", "-..-", "-.--",
        "--..",
    ];
    word.chars()
        .map(|c| morse_dictionary[(c as i32 - 'a' as i32) as usize])
        .collect()
}