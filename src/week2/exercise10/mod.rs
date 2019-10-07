use std::collections::HashMap;
pub fn word_pattern(pattern: String, words: String) -> bool {
    let pattern = pattern.chars().collect::<Vec<char>>();
    let words = words.split(' ').collect::<Vec<&str>>();
    (pattern.len() == words.len())
        && build_map_pattern(&pattern, &words)
        && build_map_words(&words, &pattern)
}

fn build_map_pattern(keys: &Vec<char>, values: &Vec<&str>) -> bool {
    let mut map: HashMap<char, &str> = HashMap::new();
    keys.iter().zip(values.iter()).all(|(k, v)| {
        if map.get(&k) == None {
            map.insert(*k, *v);
            true
        } else {
            map.get(&k) == Some(&v)
        }
    })
}

fn build_map_words(keys: &Vec<&str>, values: &Vec<char>) -> bool {
    let mut map: HashMap<&str, char> = HashMap::new();
    keys.iter().zip(values.iter()).all(|(k, v)| {
        if map.get(*k) == None {
            map.insert(*k, *v);
            true
        } else {
            map.get(*k) == Some(&v)
        }
    })
}
