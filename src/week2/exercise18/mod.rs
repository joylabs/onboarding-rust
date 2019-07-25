use std::collections::HashMap;
use std::iter::FromIterator;

pub fn frequency_sort(s: String) -> String {
      let mut letter_map = HashMap::new();

      s.chars().for_each(|c| {
            let count = letter_map.entry(c).or_insert(0);
            *count += 1;
      });

      let mut letter_vec = Vec::from_iter(letter_map);
      letter_vec.sort_by(|&(_, a), &(_, b)| b.cmp(&a));

      letter_vec.iter().fold("".to_string(), |mut acc, (letter, value)| {
            (0..*value).for_each(|_| acc.push(*letter));
            acc
      })
}
