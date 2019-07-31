use std::collections::HashSet;

#[derive(Default)]
pub struct Trie {
      elements: HashSet<String>,
      element_parts: HashSet<String>,
}

impl Trie {
      pub fn new() -> Self {
            Self {
                  elements: HashSet::new(),
                  element_parts: HashSet::new(),
            }
      }

      pub fn insert(&mut self, word: String) {
            let word = word.chars().fold("".to_string(), |mut acc, c| {
                  acc.push(c);
                  self.element_parts.insert(acc.clone());
                  acc
            });
            self.elements.insert(word);
      }

      pub fn search(&self, word: String) -> bool {
            self.elements.contains(&word)
      }

      pub fn starts_with(&self, prefix: String) -> bool {
            self.element_parts.contains(&prefix)
      }
}
