use std::collections::HashMap;

pub fn find_anagrams(s: &str, p: &str) -> Vec<i32> {
      let mut anagram_index: Vec<i32> = Vec::new();
      let s_length = s.len();
      let p_len = p.len();

      if s_length < p_len {
            return anagram_index;
      }

      for mut i in 0..=(s_length - p_len) {
            let j = i + p_len;
            println!("p = {:?}, slice = {}, index = {}", p, &s[i..j], i);
            if contains_all(&s[i..j], p) {
                  anagram_index.push(i as i32);
                  i = j + 1;
            }
      }

      anagram_index
}

fn contains_all(s: &str, p: &str) -> bool {
      // let mut s: Vec<char> = s.chars().collect();

      // for x in p.chars() {
      //       if s.contains(&x) {
      //             if let Some(item) = s.iter().position(|&y| y == x) {
      //                   s.remove(item);
      //             }
      //       }
      // }

      // s.is_empty()
      let mut map1 = HashMap::new();
      let mut map2 = HashMap::new();

      for (x, y) in s.chars().zip(p.chars()) {
            let count1 = map1.entry(x).or_insert(0);
            *count1 += 1;
            let count2 = map2.entry(y).or_insert(0);
            *count2 += 1;
      }

      println!("map1: {:?}, map2: {:?}", map1, map2);

      map1 == map2
}
