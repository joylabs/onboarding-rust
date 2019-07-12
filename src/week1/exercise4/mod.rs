pub fn self_dividing_numbers(left: i32, right: i32) -> Vec<i32> {
    (left..=right)
        .filter(|n| is_self_dividing(*n))
        .collect::<Vec<i32>>()
}

fn is_self_dividing(n: i32) -> bool {
    n.to_string()
        .chars()
        .map(|c| c as i32 - '0' as i32)
        .all(|d| d != 0 && n % d == 0)
}
