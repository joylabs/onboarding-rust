use std::collections::HashSet;

pub fn special_equivalent(input: Vec<String>) -> i32 {

    let mut groups: HashSet<String> = HashSet::new();

    input.iter().for_each(|s| {
        let mut vectors = get_vectors(s.to_string());
        vectors.0.sort();
        vectors.1.sort();
        vectors.0.append(&mut vectors.1);
        groups.insert(vectors.0.into_iter().collect());
    });

    groups.len() as i32
}

fn get_vectors(s: String) -> (Vec<char>, Vec<char>) {
    (
        s.chars().step_by(2).collect(),
        s.chars().skip(1).step_by(2).collect(),
    )
}