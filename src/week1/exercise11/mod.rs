pub fn reverse_words(input: &str) -> String {
   input
      .split_whitespace()
      .map(|w| w.chars().rev().collect::<String>())
      .fold("".to_string(), |mut acc, x| {
         if !acc.is_empty() {
            acc.push_str(" ");
         }
         acc.push_str(&x);
         acc
      })
}