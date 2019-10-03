use std::collections::HashSet;
use std::iter::FromIterator;

pub fn num_special_equiv_groups(a: Vec<String>) -> i32 {
    a.into_iter()
        .fold(&mut HashSet::new(), |acc, word| {
            acc.insert(sort_string(word));
            acc
        })
        .len() as i32
}

fn sort_string(word: String) -> String {
    let mut evens: Vec<_> = word
        .chars()
        .enumerate()
        .filter(|(i, _c)| i % 2 == 0)
        .map(|(_i, c)| c)
        .collect();

    let mut odds: Vec<_> = word
        .chars()
        .enumerate()
        .filter(|(i, _c)| i % 2 == 1)
        .map(|(_i, c)| c)
        .collect();
    evens.sort();
    odds.sort();
    format!("{}{}", String::from_iter(evens), String::from_iter(odds))
}
