use std::collections::HashMap;

#[derive(Default)]
pub struct Trie {
      children: HashMap<char, Box<Trie>>,
      is_word: bool,
}

impl Trie {
      pub fn new() -> Self {
            Trie {
                  children: HashMap::new(),
                  is_word: false,
            }
      }

      pub fn insert(&mut self, word: String) {
        let mut nodo = self;
             for ch in word.chars() {
                  nodo = nodo
                        .children
                        .entry(ch)
                        .or_insert_with(|| Box::new(Trie::new()));
            }
            nodo.is_word = true;
      }

      pub fn search(&self, word: String) -> bool {
            let mut nodo = self;
            for ch in word.chars() {
                  if nodo.children.contains_key(&ch) {
                        nodo = nodo.children.get(&ch).unwrap();
                  } else {
                        return false;
                  }
            }
            if nodo.is_word {
                  return true;
            }
            false
      }

      pub fn starts_with(&self, prefix: String) -> bool {
            let mut nodo = self;
            for ch in prefix.chars() {
                  if nodo.children.contains_key(&ch) {
                        nodo = nodo.children.get(&ch).unwrap();
                  } else {
                        return false;
                  }
            }
            true
      }
}