use std::collections::HashSet;

pub fn find_error_nums(nums: Vec<i32>) -> Vec<i32> {
    let mut set = HashSet::new();

    let expected_sum: i32 = (nums.len() * (nums.len() + 1) / 2) as i32;
    let sum: i32 = nums.iter().sum();

    nums.iter()
        .find(|item| {
            if set.get(item).is_some() {
                true
            } else {
                set.insert(*item);
                false
            }
        })
        .map(|item| vec![*item, expected_sum - (sum - item)])
        .unwrap()
}

