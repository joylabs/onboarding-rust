pub fn is_power_of_two(n: i32) -> bool {
        (0..31).map(|n| 1 << n).any(|p| p == n)
}
