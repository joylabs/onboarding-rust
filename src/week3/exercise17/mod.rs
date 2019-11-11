use std::collections::HashMap;

pub fn min_distance(word1: String, word2: String) -> i32 {
    let mut mem_matches: HashMap<(usize, usize), i32> = HashMap::new();
    // Base cases
    (0..=word1.len()).for_each(|i| {
        mem_matches.insert((i, 0), i as i32);
    });
    (0..=word2.len()).for_each(|j| {
        mem_matches.insert((0, j), j as i32);
    });

    (1..=word1.len()).for_each(|i| {
        (1..=word2.len()).for_each(|j| {
            if word1[(i - 1)..i] == word2[(j - 1)..j] {
                let prev_val = *mem_matches.get(&(i - 1, j - 1)).unwrap();
                mem_matches.insert((i, j), prev_val);
            } else {
                let mut possible_changes: Vec<i32> = Vec::with_capacity(3);
                possible_changes.push(1 + mem_matches.get(&(i, j - 1)).unwrap());
                possible_changes.push(1 + mem_matches.get(&(i - 1, j)).unwrap());
                possible_changes.push(1 + mem_matches.get(&(i - 1, j - 1)).unwrap());

                mem_matches.insert((i, j), *possible_changes.iter().min().unwrap());
            }
        });
    });

    *mem_matches.entry((word1.len(), word2.len())).or_insert(0)
}
