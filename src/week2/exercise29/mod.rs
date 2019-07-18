use std::collections::HashSet;

pub fn special_equivalent(input: Vec<String>) -> i32 {

    let mut groups: HashSet<String> = HashSet::new();

    input.iter().for_each(|s| {
        let (mut even, mut odd) = get_vectors(s.to_string());
        even.sort();
        odd.sort();
        even.append(&mut odd);
        groups.insert(even.into_iter().collect());
    });
    println!("{:?}", groups);
    groups.len() as i32
}

fn get_vectors(s: String) -> (Vec<char>, Vec<char>) {
    (
        s.chars().step_by(2).collect(),
        s.chars().skip(1).step_by(2).collect(),
    )
}