use std::collections::HashMap;

pub fn min_distance(word1: String, word2: String) -> i32 {
    if word1.is_empty() {
        return word2.len() as i32;
    } else if word2.is_empty() {
        return word1.len() as i32;
    } else if word1 == word2 {
        return 0;
    }
    
    let mut mem_matches: HashMap<usize, i32> = HashMap::new();
    find_best_pattern(0, 0, &word1, &word2, 0, 0, &mut mem_matches);

    println!("mem_matches: {:?}", mem_matches);

    if let Some(matches) = mem_matches.values().max() {
        if word1.len() >= word2.len() {
            (word1.len() as i32 - matches).abs()
        } else {
            (word2.len() as i32 - matches).abs()
        }
    } else {
        (word1.len() as i32 - word2.len() as i32).abs()
    }
}

fn find_best_pattern(i: usize, j: usize, word1: &str, word2: &str, match_count: i32, first_position: usize, mem_matches: &mut HashMap<usize, i32>) {
    println!("i: {}, j: {}, word1: {}, word2: {}, match_count: {}, first_pos: {}, mem_matches: {:?}", i, j, word1, word2, match_count, first_position, mem_matches);
    if i == word1.len() || first_position == word2.len() || j == word2.len() {
        mem_matches.insert(first_position, match_count);
        return;
    }

    if let Some(position) = find_letter_position(&word1[i..], &word2[j..=j]) {
        println!("found");
        find_best_pattern(position + 1, j + 1, word1, word2, match_count + 1, first_position, mem_matches);
    } else {
        println!("not found");
        mem_matches.insert(first_position, match_count);
        find_best_pattern(0, first_position + 1, word1, word2, 0, first_position + 1, mem_matches);
    }
}

fn find_letter_position(word: &str, letter: &str) -> Option<usize> {
    if !word.contains(letter) {
        None
    } else {
        Some(
            word.char_indices()
                .fold((0, false), |(position, found), (i, c)| {
                    if !found && &(c.to_string()) == letter {
                        (i, true)
                    } else {
                        (position, found)
                    }
                })
                .0,
        )
    }
}