use std::collections::HashMap;

pub fn num_jewels_in_stones(j: String, s: String) -> i32 {
    let mut hashmap: HashMap<char, i32> = HashMap::new();

    s.chars().for_each(|ch| {
        let count = hashmap.entry(ch).or_insert(0);
        *count += 1;
    });

    j.chars().fold(0, |mut jewels, ch| {
        jewels += *hashmap.entry(ch).or_insert(0);
        jewels
    })
}
