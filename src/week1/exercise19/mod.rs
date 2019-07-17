use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let numbers: HashMap<_, _> = nums.iter().enumerate().map(|(i, n)| (n, i)).collect();
    nums.iter()
        .enumerate()
        .find(|(i, n)| numbers.get(&(target - *n)).map_or(false, |v| v != i))
        .map(|(i, n)| vec![i as i32, numbers[&(target - *n)] as i32])
        .unwrap()
}