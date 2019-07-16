#[derive(Default, Debug)]
pub struct MyHashSet {
      key: Vec<i32>,
}


impl MyHashSet {
      pub fn new() -> Self {
            Self { key: Vec::new() }
      }

      pub fn add(&mut self, key: i32) {
            match self.key.iter().position(|x| *x == key) {
                  None => {
                        self.key.push(key);
                  }
                  _ => {
                        println!("Value {} already exists", key);
                  }
            };
      }

      pub fn remove(&mut self, key: i32) {
            match self.key.iter().position(|x| *x == key) {
                  Some(i) => {
                        self.key.remove(i);
                  }
                  None => {
                        println!("No element {} to delete", key);
                  }
            };
      }

      pub fn contains(&self, key: i32) -> bool {
            self.key.iter().any(|x| *x == key)
      }
}
