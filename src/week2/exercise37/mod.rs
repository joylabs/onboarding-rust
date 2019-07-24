use std::collections::HashMap;

pub fn find_the_difference(s: &str, t: &str) -> char {
    let mut map = HashMap::new();
    let last_char = t.to_string().pop().unwrap();
    map.insert(last_char, -1);

    for (s, t) in s.chars().zip(t.chars()) {
        let sum = map.entry(s).or_insert(0);
        *sum += 1;
        let less = map.entry(t).or_insert(0);
        *less -= 1;
    }

    map.into_iter()
        .find(|(_, i)| *i == -1)
        .map(|(ch, _)| ch)
        .unwrap()
}
