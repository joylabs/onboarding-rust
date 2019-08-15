use std::collections::HashMap;

pub fn num_jewels_in_stones(j: String, s: String) -> i32 {
    let jewels_and_stones: Vec<char> = s.chars().collect();
    let my_jewels: Vec<char> = j.chars().collect();

    let mut map_jewels_and_stones: HashMap<char, i32> = HashMap::new();

    jewels_and_stones.into_iter().for_each(|c| {
        let count = map_jewels_and_stones.entry(c).or_insert(0);
        *count += 1;
    });

    my_jewels.iter().fold(0, |acc, c| {
        let count = match map_jewels_and_stones.get(&c) {
            Some(x) => *x,
            None => 0,
        };
        acc + count
    })

}