#[derive(Default)]
pub struct MyHashSet {
    elements: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyHashSet {
    /** Initialize your data structure here. */
    pub fn new() -> Self {
        Self {
            elements: Vec::new(),
        }
    }

    pub fn add(&mut self, key: i32) {
        if !self.elements.contains(&key) {
            self.elements.push(key);
        }
    }

    pub fn remove(&mut self, key: i32) {
        if self.elements.contains(&key) {
            let index = self.elements.iter().position(|&x| x == key).unwrap();
            self.elements.remove(index);
        }
    }
    /** Returns true if this set contains the specified element */
    pub fn contains(&self, key: i32) -> bool {
        self.elements.contains(&key)
    }
}
