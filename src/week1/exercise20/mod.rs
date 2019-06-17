
use std::collections::hash_map::Entry;
use std::collections::HashMap;

pub fn exercise20(v: Vec<i32>, x: i32) -> Vec<i32> {

    let mut key = HashMap::new();
    for (i, item) in v.iter().enumerate() {
        key.insert(*item, i);
    }

    for (j, item) in v.iter().enumerate() {
        let minus = x - item;
        if let Entry::Occupied(entry) = key.entry(minus) {
            let x = *entry.get() as i32;
            if x != j as i32 {
                return vec![j as i32, x];
            }
        }
    }
    vec![0, 0]
}

