pub fn first_uniq_char(s: String) -> i32 {
    let input: Vec<char> = s.chars().collect();
    let mut char_list: Vec<char> = vec![];
    let mut index_char_list: Vec<(usize, char)> = vec![];

    for (i, c) in input.iter().enumerate() {
        if !char_list.iter().any(|e| e == c) {
            char_list.push(*c);
            index_char_list.push((i, *c));
        } else {
            index_char_list.retain(|e| e.1 != *c);
        }
    }

    if !index_char_list.is_empty() {
        index_char_list[0].0 as i32
    } else {
        -1
    }
}