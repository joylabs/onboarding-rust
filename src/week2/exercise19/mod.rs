pub fn find_error_nums(nums: Vec<i32>) -> Vec<i32> {
      let correct_nums: Vec<i32> = ((1 as i32)..=(nums.len() as i32)).collect();
      let mut nums = nums.clone();
      nums.sort();

      let mut duplicated_num = 0;
      let mut missing_num = 0;
      for (i, n) in correct_nums.iter().enumerate() {
            if !nums.contains(n) {
                  missing_num = *n;
            }

            if i + 1 < nums.len() && nums[i] == nums[i + 1] {
                  duplicated_num = nums[i];
            }
      }

      vec![duplicated_num, missing_num]
}
