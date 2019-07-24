#[derive(Default, Debug)]
pub struct MyHashMap {
    map: Vec<(i32, i32)>,
}

impl MyHashMap {

    pub fn new() -> Self {
        Self { map: Vec::new() }
    }

    pub fn put(&mut self, key: i32, value: i32) {
        match self.map.iter().position(|key_m| key_m.0 == key) {
            Some(i) => self.map[i].1 = value,
            None => self.map.push((key, value)),
        }
    }

    pub fn get(&self, key: i32) -> i32 {
        match self.map.iter().find(|map| map.0 == key) {
            Some(i) => i.1,
            None => -1,
        }
    }

    pub fn remove(&mut self, key: i32) {
        if let Some(i) = self.map.iter().position(|map| map.0 == key) {
            self.map.remove(i);
        }
    }
}

