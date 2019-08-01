use std::collections::HashMap;

pub fn first_unique_char(input: String) -> i32 {
    let mut char_map = HashMap::new();

    input.chars().for_each(|c| {
        let counter = char_map.entry(c).or_insert(0);
        *counter += 1;
    });

    for (i, c) in input.char_indices() {
        if let Some(1) = char_map.get(&c) {
            return i as i32;
        }
    }
    -1
}

pub fn first_unique_char_2(input: String) -> i32 {
    let mut char_map = HashMap::new();

    for c in input.chars() {
        let counter = char_map.entry(c).or_insert(0);
        *counter += 1;
    }

    let unique_char_map: HashMap<char, i32> = char_map.into_iter().filter(|x| x.1 == 1).collect();

    for (i, c) in input.char_indices() {
        if unique_char_map.contains_key(&c) {
            return i as i32;
        }
    }

    -1
}

pub fn first_unique_char_3(input: String) -> i32 {
    let mut char_map = HashMap::new();

    input.chars().enumerate().for_each(|(i, c)| {
        let counter = char_map.entry(c).or_insert((0, i as i32));
        counter.0 += 1;
    });

    if let Some(i) = char_map
        .into_iter()
        .filter(|x| (x.1).0 == 1)
        .map(|x| (x.1).1)
        .collect::<Vec<i32>>()
        .into_iter()
        .min()
    {
        i
    } else {
        -1
    }
}
