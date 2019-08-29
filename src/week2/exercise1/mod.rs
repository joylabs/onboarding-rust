pub fn jewels_and_stones(jewels: String, stones: String) -> i32 {
    stones
        .chars()
        .filter(|c| jewels.contains(*c))
        .fold(0, |acc, _| acc + 1)
}
