use std::collections::HashMap;

pub fn isomorphic_string(s: String, t: String) -> bool {
    suma(s) == suma(t)
}
fn suma(s: String) -> String {
    let mut map = HashMap::new();

    s.chars()
        .enumerate()
        .map(|(i, ch)| map.entry(ch).or_insert(i).to_string())
        .collect::<String>()
}