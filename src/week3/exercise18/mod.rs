use std::cmp::max;

pub fn rob(nums: Vec<i32>) -> i32 {
    if nums.len() == 1 {
        return nums[0];
    }

    let odd_houses = nums
        .iter()
        .skip(1)
        .fold((0, 0), |(current, previous), house| {
            (max(house + previous, current), current)
        })
        .0;
    let even_houses = nums
        .iter()
        .rev()
        .skip(1)
        .fold((0, 0), |(current, previous), house| {
            (max(house + previous, current), current)
        })
        .0;

    max(odd_houses, even_houses)
}
