use std::collections::HashMap;

pub fn word_pattern(pattern: String, str: String) -> bool {
   get_pattern(pattern) == check_pattern(str)
}

fn get_pattern(word: String) -> String {
   let mut map = HashMap::new();

   word
      .chars()
      .enumerate()
      .map(|(i, ch)| map.entry(ch).or_insert(i).to_string())
      .collect::<String>()
}

fn check_pattern(sentence: String) -> String {
   let mut map = HashMap::new();

   sentence
      .split_whitespace()
      .enumerate()
      .map(|(i, ch)| map.entry(ch).or_insert(i).to_string())
      .collect::<String>()
}