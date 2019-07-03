use std::collections::HashSet;

/*
    First attempt
    pub fn special_equivalent(input: Vec<String>) -> i32 {
        // Iterate over input vector and get strings
        let mut groups: HashSet<String> = HashSet::new();
        for s in input {
            let mut current_string_even: Vec<char> = vec![];
            let mut current_string_odd: Vec<char> = vec![];
            // Iterate over string
            for (i, c) in s.chars().enumerate() {
                // Divide into odds and evens
                if i % 2 == 0 {
                    current_string_even.push(c);
                } else {
                    current_string_odd.push(c);
                }
            }
            // Sort substrings
            current_string_even.sort();
            current_string_odd.sort();
            current_string_even.append(&mut current_string_odd);
            groups.insert(current_string_even.into_iter().collect());
        }
        groups.len() as i32
    }
*/

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
        s.chars()
            .enumerate()
            .filter(|(i, _)| i % 2 == 0)
            .map(|(_, c)| c)
            .collect(),
        s.chars()
            .enumerate()
            .filter(|(i, _)| i % 2 != 0)
            .map(|(_, c)| c)
            .collect(),
    )
}