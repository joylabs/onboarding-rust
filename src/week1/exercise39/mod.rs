pub fn find_the_difference(s: String, t: String) -> char {
    let mut s = s.chars().collect::<Vec<char>>();
    s.sort();
    let mut t = t.chars().collect::<Vec<char>>();
    t.sort();

    let result = s
        .iter()
        .zip(t.iter())
        .skip_while(|(a, b)| a == b)
        .collect::<Vec<(&char, &char)>>();

    if result.is_empty() {
        *t.last().unwrap()
    } else {
        *result[0].1
    }

}
