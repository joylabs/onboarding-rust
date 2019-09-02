use std::collections::HashMap;
pub fn two_sum(input: Vec<i32>, target: i32) -> Vec<i32> {
    let numbers: HashMap<_, _> = input.iter().enumerate().map(|(i, n)| (n, i)).collect();
    input
        .iter()
        .enumerate()
        .find(|(i, n)| numbers.get(&(target - *n)).map_or(false, |v| v != i))
        .map(|(i, n)| vec![i as i32, numbers[&(target - *n)] as i32])
        .unwrap()
}