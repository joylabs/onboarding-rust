use std::collections::HashSet;

  pub fn unique_morse_representations(words: Vec<String>) -> i32 {
     const MORSE_CODE: [&str; 26] = [
         ".-", "-...", "-.-.", "-..", ".", "..-.", "--.", "....", "..", ".---", "-.-", ".-..", "--",
         "-.", "---", ".--.", "--.-", ".-.", "...", "-", "..-", "...-", ".--", "-..-", "-.--",
         "--..",
     ];

      let morse_set: HashSet<String> = words
         .iter()
         .map(|word| {
             word.chars()
                 .fold(String::new(), |acc, c| acc + MORSE_CODE[c as usize - 97])
         })
         .collect();

      morse_set.len() as i32
 }

  pub fn unique_morse_representations_2(words: Vec<String>) -> i32 {
     const MORSE_CODE: [&str; 26] = [
         ".-", "-...", "-.-.", "-..", ".", "..-.", "--.", "....", "..", ".---", "-.-", ".-..", "--",
         "-.", "---", ".--.", "--.-", ".-.", "...", "-", "..-", "...-", ".--", "-..-", "-.--",
         "--..",
     ];

      let mut morse_set = HashSet::new();

      for word in words {
         let mut morse_string = String::new();
         for c in word.chars() {
             let index = c as usize - 97;
             morse_string += MORSE_CODE[index];

          }
         morse_set.insert(morse_string);
     }

      morse_set.len() as i32
 }

  pub fn unique_morse_representations_3(words: Vec<String>) -> i32 {
     const MORSE_CODE: [&str; 26] = [
         ".-", "-...", "-.-.", "-..", ".", "..-.", "--.", "....", "..", ".---", "-.-", ".-..", "--",
         "-.", "---", ".--.", "--.-", ".-.", "...", "-", "..-", "...-", ".--", "-..-", "-.--",
         "--..",
     ];

        words
          .iter()
          .fold(&mut HashSet::new(), |acc, word| {
              acc.insert(
                  word.chars()
                      .fold(String::new(), |acc2, c| acc2 + MORSE_CODE[c as usize - 97]),
              );
              acc
          })
          .len() as i32
 }
 