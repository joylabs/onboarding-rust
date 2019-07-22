use std::collections::HashMap;

pub fn first_unique_char(input: String) -> i32 {
      let mut char_map = HashMap::new();

      input.chars().for_each(|c| {
            let counter = char_map.entry(c).or_insert(0);
            *counter += 1;
      });

      let unique_char_map: HashMap<char, i32> = char_map.into_iter().filter(|x| x.1 == 1).collect();

      for (i, c) in input.char_indices() {
            if unique_char_map.contains_key(&c) {
                  return i as i32;
            }
      }

      -1
}
