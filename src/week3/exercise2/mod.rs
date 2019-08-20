pub fn is_subsequence(s: String, t: String) -> bool {
    if s.is_empty() {
        return true;
    }

    let s: Vec<char> = s.chars().map(|ch| ch).collect();
    let t = t.chars().fold(0, |mut acc, ch| {
        if acc < s.len() && ch == s[acc] {
            acc += 1;
        }
        acc
    });

    t == s.len()
}
