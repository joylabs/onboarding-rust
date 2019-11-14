use std::collections::HashMap;
pub fn frequency_sort(s: String) -> String {
    let mut count: HashMap<char, u32> = HashMap::new();
    s.chars().for_each(|c| *count.entry(c).or_insert(0) += 1);

    let mut count_vec: Vec<(&char, &u32)> = count.iter().collect();
    count_vec.sort_by(|a, b| b.1.cmp(a.1));

    count_vec.iter().fold(String::new(), |mut acc, (c, n)| {
        acc = format!("{}{}", acc, return_repeated_char(**c, **n));
        acc
    })
}

fn return_repeated_char(c: char, n: u32) -> String {
    std::iter::repeat(c).take(n as usize).collect::<String>()
}