use std::collections::HashMap;

pub fn min_distance(word1: String, word2: String) -> i32 {
    let mut mem_matches: HashMap<(usize, usize), i32> = HashMap::new();
    find_best_pattern(0, 0, &word1, &word2, &mut mem_matches)
}

fn find_best_pattern(
    i: usize,
    j: usize,
    word1: &str,
    word2: &str,
    mem_matches: &mut HashMap<(usize, usize), i32>,
) -> i32 {
    println!("i: {}, j: {}", i, j);
    if i == word1.len() && j == word2.len() {
        println!("base");
        0
    } else if mem_matches.contains_key(&(i, j)) {
        println!("memo");
        *mem_matches.get(&(i, j)).unwrap()
    } else if i == word1.len() {
        println!("end of word1");
        1 + find_best_pattern(i, j + 1, word1, word2, mem_matches)
    } else if j == word2.len() {
        println!("end of word2");
        1 + find_best_pattern(i + 1, j, word1, word2, mem_matches)
    } else if word1[i..=i] == word2[j..=j] {
        println!("equal");
        find_best_pattern(i + 1, j + 1, word1, word2, mem_matches)
    } else {
        println!("not the end of any word, not equal");
        let mut possible_changes: Vec<i32> = Vec::with_capacity(3);
        possible_changes.push(1 + find_best_pattern(i, j + 1, word1, word2, mem_matches));
        possible_changes.push(1 + find_best_pattern(i + 1, j, word1, word2, mem_matches));
        possible_changes.push(1 + find_best_pattern(i + 1, j + 1, word1, word2, mem_matches));

        println!("({}, {}) - possible changes: {:?}", i, j, possible_changes);

        *mem_matches
            .entry((i, j))
            .or_insert(*possible_changes.iter().min().unwrap())
    }
}