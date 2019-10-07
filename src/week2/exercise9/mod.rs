use std::collections::HashMap;
pub fn is_isomorphic(s: String, t: String) -> bool {
    build_map(&s, &t) && build_map(&t, &s)
}

fn build_map(keys: &str, values: &str) -> bool {
    let mut map: HashMap<char, char> = HashMap::new();
    keys.chars().zip(values.chars()).all(|(k, v)| {
        if !map.contains_key(&k) {
            map.insert(k, v);
            true
        } else {
            map.get(&k) == Some(&v)
        }
    })
}
