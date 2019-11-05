use std::collections::HashMap;
pub fn first_uniq_char(s: String) -> i32 {
    let mut all_letters = HashMap::new();
    s.chars().for_each(|c| {
        let counter = all_letters.entry(c).or_insert(0);
        *counter += 1;
    });
    let non_repeating_characters: HashMap<char, i32> = all_letters
        .into_iter()
        .filter(|(_, val)| *val == 1)
        .collect();

    for (i, c) in s.char_indices() {
        if non_repeating_characters.contains_key(&c) {
            return i as i32;
        }
    }
    -1
}

// pub fn first_uniq_char2(s: String) -> i32 {
//     let s2 = s.clone();
//     for (i, c) in s.chars().enumerate() {
//         if s2.matches(c).count() == 1 {
//             return i as i32;
//         }
//     }
//     -1
// }
