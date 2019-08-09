#[derive(Default)]
pub struct MyHashMap {
      key: Vec<i32>,
      value: Vec<i32>,
}

impl MyHashMap {
      pub fn new() -> Self {
            Self {
                  key: Vec::new(),
                  value: Vec::new(),
            }
      }

      pub fn put(&mut self, key: i32, value: i32) {
            if let Some(i) = self.key.iter().position(|x| *x == key) {
                  self.value[i] = value;
            } else {
                  self.key.push(key);
                  self.value.push(value);
            };
      }

      pub fn get(&self, key: i32) -> i32 {
            if let Some(i) = self.key.iter().position(|x| *x == key) {
                  self.value[i]
            } else {
                  -1
            }
      }

      pub fn remove(&mut self, key: i32) {
            if let Some(i) = self.key.iter().position(|x| *x == key) {
                  self.value.remove(i);
                  self.key.remove(i);
            } else {
                  //
            };
      }
}
