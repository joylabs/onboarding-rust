pub fn jewels_in_stones(j: String, s: String) -> i32 {
    s.chars()
        .filter(|s| j.contains(*s))
        .collect::<String>()
        .len() as i32
}