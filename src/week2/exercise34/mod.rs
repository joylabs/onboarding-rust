#[derive(Default, Debug)]
pub struct MyHashSet {
    key: Vec<i32>,
}

impl MyHashSet {
    pub fn new() -> Self {
        Self { key: Vec::new() }
    }
    pub fn add(&mut self, key: i32) {
        if self.key.iter().position(|x| *x == key).is_none() {
            self.key.push(key);
        }
    }
    pub fn remove(&mut self, key: i32) {
        if let Some(i) = self.key.iter().position(|x| *x == key) {
            self.key.remove(i);
        }
    }
    pub fn contains(&self, key: i32) -> bool {
        self.key.iter().any(|x| *x == key)
    }
}