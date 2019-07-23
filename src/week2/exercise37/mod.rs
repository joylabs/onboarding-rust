use std::collections::HashMap;

pub fn find_the_difference(s: &str, t: &str) -> char {

    let mut map = HashMap::new();
    t.chars().for_each(|ch| {
        let count = map.entry(ch).or_insert(0);
        *count += 1;
    });
    for ch in s.chars() {
        let value = map.entry(ch).or_insert(0);
        if *value != 0 {
            *value -= 1;
        } else {
            return ch;
        }
    }

    map.into_iter()
        .find(|(_, i)| *i != 0)
        .map(|(ch, _)| ch)
        .unwrap()
}
