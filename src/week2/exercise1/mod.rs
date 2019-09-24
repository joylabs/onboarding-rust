pub fn jewels_in_stones(jewels: String, stones: String) -> i32 {
    stones
        .chars()
        .filter(|s| jewels.contains(*s))
        .collect::<String>()
        .len() as i32
}
