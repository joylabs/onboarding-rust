use std::collections::HashMap;

pub fn find_the_difference(s: String, t: String) -> char {
      let mut t_char_map = HashMap::new();

      t.chars().for_each(|c| {
            let counter = t_char_map.entry(c).or_insert(0);
            *counter += 1;
      });

      s.chars().for_each(|c| {
            t_char_map.entry(c).and_modify(|e| *e -= 1);
      });

      t_char_map.iter().fold('0', |mut acc, (&k, &v)| {
            if v == 1 {
                  acc = k
            }
            acc
      })
}
