pub fn valid_anagram(s: String, t: String) -> bool {

    let s: Vec<char> = s.chars().collect();
    let t: Vec<char> = t.chars().collect();

    if s.len() != t.len() {
        return false;
    }

    let mut char_counter = [0; 26];
    for (i, c) in s.into_iter().enumerate() {
        char_counter[((c as i32) - ('a' as i32)) as usize] += 1;
        char_counter[((t[i] as i32) - ('a' as i32)) as usize] -= 1;
    }
    for i in char_counter.iter() {
        if *i != 0 {
            return false;
        }
    }

    true
}