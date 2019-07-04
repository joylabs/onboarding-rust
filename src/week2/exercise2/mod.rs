use std::collections::HashSet;

pub fn count_unique_morse_words(input: Vec<&str>) -> i32 {
   transform_into_morse(input)
      .into_iter()
      .collect::<HashSet<String>>()
      .len() as i32
}

fn transform_into_morse(words: Vec<&str>) -> Vec<String> {
   let morse_code = vec![
      ".-", "-...", "-.-.", "-..", ".", "..-.", "--.", "....", "..", ".---", "-.-", ".-..", "--",
      "-.", "---", ".--.", "--.-", ".-.", "...", "-", "..-", "...-", ".--", "-..-", "-.--", "--..",
   ];

   let alphabet = vec![
      'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
      's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
   ];

   words
      .into_iter()
      .map(|w| {
         w.chars()
            .map(|c| alphabet.iter().position(|x| *x == c).unwrap())
            .fold("".to_string(), |morse, c| morse + morse_code[c])
      })
      .collect()
}

