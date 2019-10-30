pub fn find_the_difference(s: String, t: String) -> char {
    let mut s = s.chars().collect::<Vec<char>>();
    s.sort();
    let mut t = t.chars().collect::<Vec<char>>();
    t.sort();

    for (i, c) in s.iter().enumerate() {
        if *c != t[i] {
            return t[i];
        }
    }
    t[t.len() - 1]
}
