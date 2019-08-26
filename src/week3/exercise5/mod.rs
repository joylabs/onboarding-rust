
pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    if let Some(value) = nums.iter().position(|&x| x as i32 == target) {
        return value as i32;
    }
    -1
}

// fn binary_search(nums: Vec<i32>, target: i32) {
//     let mut max = nums.len() - 1;
//     let mut min = 0;
//     while min < max {
//         let position = min + (max - min) / 2;
//         println!("val = {}", position);
//         if nums[position] == target {
//             println!("position {}", position);
//             break;
//         } else if nums[position] < target {
//             min = position + 1;
//         } else {
//             max = position;
//         }
//     }
// }
