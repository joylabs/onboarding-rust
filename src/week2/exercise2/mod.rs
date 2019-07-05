use std::collections::HashSet;
use std::collections::HashMap;

pub fn count_unique_morse_words(input: Vec<&str>) -> i32 {
   transform_into_morse(input)
      .into_iter()
      .collect::<HashSet<String>>()
      .len() as i32
}

fn transform_into_morse(words: Vec<&str>) -> Vec<String> {
   let morse_code: Vec<&str> = vec![
      ".-", "-...", "-.-.", "-..", ".", "..-.", "--.", "....", "..", ".---", "-.-", ".-..", "--",
      "-.", "---", ".--.", "--.-", ".-.", "...", "-", "..-", "...-", ".--", "-..-", "-.--", "--..",
   ];

   let alphabet: Vec<char> = vec![
      'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
      's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
   ];

   let morse_map: HashMap<_,_> = alphabet.into_iter().zip(morse_code.into_iter()).collect();

   words
      .into_iter()
      .map(|w| {
         w.chars()
            .map(|c| morse_map.get(&c).unwrap())
            .fold("".to_string(), |morse, c| morse + c)
      })
      .collect()
}

