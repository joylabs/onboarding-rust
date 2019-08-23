pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    if let Some(value) = nums.iter().position(|&x| x as i32 == target) {
        return value as i32;
    }
    -1
}
