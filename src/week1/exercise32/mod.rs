use std::collections::HashMap;

pub fn is_isomorphic(s: String, t: String) -> bool {

    let mut s_vec: Vec<i32> = vec![];
    let mut t_vec: Vec<i32> = vec![];
    let mut s_hashmap: HashMap<char, i32> = HashMap::new();
    let mut t_hashmap: HashMap<char, i32> = HashMap::new();

    for (i, c) in s.chars().enumerate() {
        let position = s_hashmap.entry(c).or_insert(i as i32);
        s_vec.push(*position);
    }

    for (i, c) in t.chars().enumerate() {
        let position = t_hashmap.entry(c).or_insert(i as i32);
        t_vec.push(*position);
    }

    if s_vec == t_vec {
        return true;
    }

    false
}