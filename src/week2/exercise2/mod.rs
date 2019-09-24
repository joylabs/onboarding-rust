use std::collections::HashSet;

pub fn unique_morse_representations(words: Vec<String>) -> i32 {
    const KEY: [&str; 26] = [
        ".-", "-...", "-.-.", "-..", ".", "..-.", "--.", "....", "..", ".---", "-.-", ".-..", "--",
        "-.", "---", ".--.", "--.-", ".-.", "...", "-", "..-", "...-", ".--", "-..-", "-.--",
        "--..",
    ];

    words
        .iter()
        .fold(&mut HashSet::new(), |acc, word| {
            acc.insert(
                word.chars()
                    .fold(String::new(), |acc2, c| acc2 + KEY[c as usize - 97]),
            );
            acc
        })
        .len() as i32
}
