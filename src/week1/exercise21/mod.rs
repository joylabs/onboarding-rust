pub fn exercise21(s1: &str, s2: &str) -> bool {

    let mut s2: Vec<char> = s2.chars().collect();
    let mut s1: Vec<char> = s1.chars().collect();

    s2.sort();
    s1.sort();

    s2 == s1
}

