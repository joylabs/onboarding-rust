
pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    if let Some(value) = nums.iter().position(|&x| x as i32 == target) {
        return value as i32;
    }
    -1
}

// pub fn binary_search(nums: Vec<i32>, target: i32) -> i32 {
//     let mut max = nums.len();
//     let mut min = 0;
//     while min < max {
//         let position = (max + min) / 2;
//         if nums[position] == target {
//             return position as i32;
//         } else if nums[position] < target {
//             min = position + 1;
//         } else {
//             max = position;
//         }
//     }
//     -1
// }
