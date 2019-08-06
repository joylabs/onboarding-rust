use std::cmp::Ordering; 

pub fn longest_word(mut words: Vec<String>) -> String {
      words.sort_by(|a, b| compare_lexicographically(&b, &a));
      
      for i in 0..words.len() {
            let mut word_in_parts = String::new();
            let mut is_contained = true;
            
            for c in words[i].chars() {
                  word_in_parts.push(c);
                  if !(words.contains(&word_in_parts)) {
                        is_contained = false;
                        break;
                  }
            }

            if is_contained {
                  return words[i].clone();
            }
      }

      "".to_string()
}

fn compare_lexicographically(s1: &str, s2: &str) -> Ordering {
      if s1.len() > s2.len() {
            Ordering::Greater
      } else if s1.len() < s2.len() {
            Ordering::Less
      } else if s1 < s2 {
            Ordering::Greater
      } else if s1 > s2 {
            Ordering::Less
      } else {
            Ordering::Equal
      }
}
