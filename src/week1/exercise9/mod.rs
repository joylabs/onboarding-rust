pub fn reverse_string(s: &mut Vec<char>) {
        let l = s.len() - 1;
        s.clone().iter().enumerate().rfold(s, |v, (i, &c)| {
                v[l - i] = c;
                v
        });
}
