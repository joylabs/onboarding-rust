use std::collections::HashMap;

pub fn is_isomorphic(s: String, t: String) -> bool {
   check_pattern(s) == check_pattern(t)
}

fn check_pattern(word: String) -> String {
   let mut map = HashMap::new();

   word.chars()
      .enumerate()
      .map(|(i, ch)| map.entry(ch).or_insert(i).to_string())
      .collect::<String>()
}
