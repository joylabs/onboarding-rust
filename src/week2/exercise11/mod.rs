#[derive(Default)]
pub struct MyHashMap {
    key: Vec<i32>,
    value: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyHashMap {
    /** Initialize your data structure here. */
    pub fn new() -> Self {
        Self {
            key: Vec::new(),
            value: Vec::new(),
        }
    }
    /** value will always be non-negative. */
    pub fn put(&mut self, key: i32, value: i32) {
        if self.key.contains(&key) {
            let index = self.key.iter().position(|&x| x == key).unwrap();
            self.value[index] = value;
        } else {
            self.key.push(key);
            self.value.push(value);
        }
    }
    /** Returns the value to which the specified key is mapped, or -1 if this map contains no mapping for the key */
    pub fn get(&self, key: i32) -> i32 {
        if self.key.contains(&key) {
            let index = self.key.iter().position(|&x| x == key).unwrap();
            self.value[index]
        } else {
            -1
        }
    }
    /** Removes the mapping of the specified value key if this map contains a mapping for the key */
    pub fn remove(&mut self, key: i32) {
        if self.key.contains(&key) {
            let index = self.key.iter().position(|&x| x == key).unwrap();
            self.key.remove(index);
            self.value.remove(index);
        }
    }
}
