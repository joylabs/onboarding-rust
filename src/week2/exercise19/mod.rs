//use itertools::Itertools;
use std::collections::HashSet;
pub fn find_error_nums(nums: Vec<i32>) -> Vec<i32> {
    let mut result = vec![];
    let repeated_number = repeated_number(&nums);
    for (i, _) in nums.iter().enumerate() {
        if !nums.contains(&(i as i32 + 1)) {
            result.push(repeated_number);
            result.push(i as i32 + 1);
            break;
        }
    }
    result
}

fn repeated_number(data: &[i32]) -> i32 {
    let set_sum: i32 = data.iter().cloned().collect::<HashSet<i32>>().iter().sum();
    let num_sum: i32 = data.iter().sum();
    num_sum - set_sum
}
