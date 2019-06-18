
use std::collections::hash_map::Entry;
use std::collections::HashMap;

pub fn exercise20(v: Vec<i32>, x: i32) -> Vec<usize> {

    let mut key = HashMap::new();
    for (i, item) in v.iter().enumerate() {
        key.insert(*item, i);
    }

    for (j, item) in v.iter().enumerate() {
        let minus = x - item;
        if let Entry::Occupied(entry) = key.entry(minus) {
            if *entry.get() != j {
                return vec![j, *entry.get()];
            }
        }
    }
    vec![0, 0]
}

