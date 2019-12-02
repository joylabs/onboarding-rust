use std::cmp::max;

pub fn rob(nums: Vec<i32>) -> i32 {
    if nums.len() == 1 {
        return nums[0];
    }
    if nums.is_empty() {
        return 0;
    }
    let mut curr_odd = 0;
    let mut previus_odd = 0;

    let mut curr_even = nums[0];
    let mut previus_even = 0;

    for (i, val) in nums.iter().skip(1).enumerate() {
        let odd_curr = curr_odd;
        curr_odd = max(previus_odd + val, odd_curr);
        previus_odd = odd_curr;

        if i < nums.len() - 2 {
            let even_curr = curr_even;
            curr_even = max(previus_even + val, even_curr);
            previus_even = even_curr;
        }
    }

    max(curr_even, curr_odd)
}
