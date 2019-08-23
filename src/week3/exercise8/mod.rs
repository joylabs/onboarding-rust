pub fn array_pair_sum(mut nums: Vec<i32>) -> i32 {
      nums.sort();
      let mut i = 0;
      let mut sum = 0;

      while i < nums.len() {
            sum += nums[i];
            i += 2;
      }

      sum
}