pub fn search(nums: Vec<i32>, target: i32) -> i32 {
      search_target(&nums, target, 0, nums.len() as i32 - 1)
}

fn search_target(nums: &[i32], target: i32, min: i32, max: i32) -> i32 {
      if min > max {
            return -1;
      }

      let index = ((max + min) as f32 / 2.0).ceil() as usize;
      // println!("min: {}, max: {}, index: {}", min, max, index);

      if nums[index] == target {
            index as i32
      } else if nums[index] < target {
            search_target(nums, target, index as i32 + 1, max)
      } else {
            search_target(nums, target, min, index as i32 - 1)
      }
}
