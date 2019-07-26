use std::collections::HashMap;

pub fn find_error_nums(nums: Vec<i32>) -> Vec<i32> {
      let mut nums_map: HashMap<i32, i32> = HashMap::new();
      let mut duplicated_num = 0;
      let mut missing_num = 0;

      for (i, n) in nums.iter().enumerate() {
            if !nums.contains(&(i as i32 + 1)) {
                  missing_num = i as i32 + 1;
            }

            let count = nums_map.entry(*n).or_insert(0);
            *count += 1;

            if *count > 1 {
                  duplicated_num = *n;
            }

            if duplicated_num > 0 && missing_num > 0 {
                  break;
            }
      }

      vec![duplicated_num, missing_num]
}
