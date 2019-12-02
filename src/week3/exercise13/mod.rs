use std::cmp;

pub fn rob(nums: Vec<i32>) -> i32 {
    let len = nums.len();

    if len == 0 {
        return 0;
    }
    if len == 1 {
        return nums[0];
    }

    cmp::max(max_rob(&nums[1..]), max_rob(&nums[..len - 1]))
}

pub fn max_rob(nums: &[i32]) -> i32 {
    let mut memo = vec![];
    memo.push(nums[0]);
    memo.push(cmp::max(nums[0], nums[1]));

    for i in 2..nums.len() {
        let sum = nums[i] + memo[i - 2];
        memo.push(cmp::max(sum, memo[i - 1]));
    }
    memo.pop().unwrap_or(0)
}
