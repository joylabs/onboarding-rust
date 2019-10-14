use std::cmp;
use std::collections::HashMap;

pub fn rob(nums: Vec<i32>) -> i32 {
    let len = nums.len();

    if len == 0 {
        return 0;
    }
    if len == 1 {
        return nums[0];
    }
    if len == 2 {
        return cmp::max(nums[0], nums[1]);
    }

    let mut map: HashMap<usize, i32> = HashMap::new();

    // with the first house
    let a = rob_dp(0, &nums[1..], &mut map);
    let b = rob_dp(1, &nums[1..], &mut map);
    let first_max = cmp::max(a, b);
    map.clear();
    // without the first house
    let a = rob_dp(0, &nums[..len - 1], &mut map);
    let b = rob_dp(1, &nums[..len - 1], &mut map);
    let second_max = cmp::max(a, b);

    cmp::max(first_max, second_max)
}

fn rob_dp(item: usize, vec: &[i32], map: &mut HashMap<usize, i32>) -> i32 {
    if map.get(&item).is_some() {
        return *map.get(&item).unwrap();
    }
    let mut a = vec[item];
    let mut b = vec[item];
    if item + 2 < vec.len() {
        a += rob_dp(item + 2, vec, map);
    }
    if item + 3 < vec.len() {
        b += rob_dp(item + 3, vec, map);
    }
    let max = cmp::max(a, b);
    map.insert(item, max);
    max
}
