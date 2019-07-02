use std::collections::HashSet;

pub fn unique_morse_representations(input: Vec<String>) -> i32 {

   let set: HashSet<String> = input
      .iter()
      .map(|word| {
         word.chars().fold("".to_string(), |morse_code, ch| {
            morse_code + &find_letter(ch)
         })
      })
      .collect();
   set.len() as i32
}

fn find_letter(ch: char) -> String {
   let position = ch.to_ascii_uppercase() as usize - 65;
   let vector_of_code = vec![
      ".-", "-...", "-.-.", "-..", ".", "..-.", "--.", "....", "..", ".---", "-.-", ".-..", "--",
      "-.", "---", ".--.", "--.-", ".-.", "...", "-", "..-", "...-", ".--", "-..-", "-.--", "--..",
   ];
   vector_of_code[position].to_string()
}
