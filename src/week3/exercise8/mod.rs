pub fn array_pair_sum(mut nums: Vec<i32>) -> i32 {
    nums.sort();
    nums.into_iter()
        .enumerate()
        .map(|(i, x)| if i % 2 == 0 { x } else { 0 })
        .sum()
}
