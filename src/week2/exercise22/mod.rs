const LETTERS_IN_ALPHABET: usize = 26;

#[derive(Default)]
pub struct Trie {
      root: TrieNode,
}

impl Trie {
      pub fn new() -> Self {
            Self {
                  root: TrieNode::new(),
            }
      }

      pub fn insert(&mut self, word: String) {
            let mut node = &mut self.root;
            
            for c in word.chars() {
                  if !node.contains_key(c) {
                        node.insert_key(c, TrieNode::new());
                  }

                  node = match node.get_node_mut(c) {
                        Some(n) => n,
                        None => panic!("Could not retrieve node..."),
                  };
            }
            node.set_end();
      }

      pub fn search(&self, word: String) -> bool {
            let node = self.search_nodes(word);

            node.is_some() && node.unwrap().get_is_end()
      }

      pub fn starts_with(&self, prefix: String) -> bool {
            self.search_nodes(prefix).is_some()
      }

      pub fn search_nodes(&self, prefix: String) -> Option<&TrieNode> {
            let mut node = &self.root;
            
            for c in prefix.chars() {
                  if node.contains_key(c) {
                        node = match node.get_node(c) {
                              Some(n) => n,
                              None => panic!("Could not retrieve node..."),
                        };
                  } else {
                        return None;
                  }
            }

            Some(node)
      }
}

#[derive(Default)]
pub struct TrieNode {
      children_nodes: Vec<Option<TrieNode>>, //Vec<Box<Node>>,
      is_end: bool,
}

impl TrieNode {
      pub fn new() -> Self {
            Self {
                  children_nodes: (0..LETTERS_IN_ALPHABET)
                        .map(|_| None)
                        .collect::<Vec<Option<TrieNode>>>(),
                  is_end: false,
            }
      }

      pub fn contains_key(&self, letter: char) -> bool {
            let index = (letter as usize) - ('a' as usize);
            self.children_nodes[index].is_some()
      }

      pub fn get_node_mut(&mut self, letter: char) -> &mut Option<Self> {
            let index = (letter as usize) - ('a' as usize);
            &mut self.children_nodes[index]
      }

      pub fn get_node(&self, letter: char) -> &Option<Self> {
            let index = (letter as usize) - ('a' as usize);
            &self.children_nodes[index]
      }

      pub fn insert_key(&mut self, letter: char, node: TrieNode) {
            let index = (letter as usize) - ('a' as usize);
            self.children_nodes[index] = Some(node);
      }

      pub fn set_end(&mut self) {
            self.is_end = true;
      }

      pub fn get_is_end(&self) -> bool {
            self.is_end
      }
}
