pub fn is_subsequence(s: String, t: String) -> bool {
    if s.is_empty() {
        return true;
    }
    let len = s.len();
    let mut s  = s.chars();
    let mut cha = s.next().unwrap();
    let t = t.chars().fold(0, |mut acc, ch| {
        if acc < len  && cha == ch{
                 acc += 1;
                 cha = s.next().unwrap_or('N');
        }
        acc
    });
    t == len
}
