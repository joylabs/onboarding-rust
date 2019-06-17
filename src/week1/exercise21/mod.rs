pub fn exercise21(s1: &str, s2: &str) -> bool {
    if s1.len() != s2.len() {
        return false;
    }

    let s2: Vec<char> = s2.chars().collect();
    let s1: Vec<char> = s1.chars().collect();

    let mut new_vector = vec![0; 256];

    for x in 0..s1.len() {
        new_vector[s1[x] as usize] += 1;
        new_vector[s2[x] as usize] -= 1;
    }

    !new_vector.into_iter().any(|x| x > 0)
}