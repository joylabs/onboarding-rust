use std::char;

pub fn is_isomorphic(s: String, t: String) -> bool {
   check_pattern(s) == check_pattern(t)
}

fn check_pattern(word: String) -> String {
   let w1: Vec<char> = word.chars().collect();
   let s2: Vec<char> = get_unique(word.chars().collect());
   let mut my_pattern = String::new();

   println!("w1 {:?}", w1);

   for (i, c) in s2.iter().enumerate() {
      for w in w1.iter() {
         if *c == *w {
            my_pattern = format!("{}{}", my_pattern, i);
         } else {
            my_pattern = format!("{}-", my_pattern);
         }
      }
   }

   println!("word {}", my_pattern);

   my_pattern
}

fn get_unique(mut word: Vec<char>) -> Vec<char> {
   let mut word2 = Vec::new();

   for c in word.iter() {
      if !word2.contains(c) {
         word2.push(*c);
      }
   }

   println!("unique {:?}", word2);

   word2
}