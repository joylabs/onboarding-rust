pub fn array_pair_sum(mut nums: Vec<i32>) -> i32 {
      nums.sort();
      nums.iter()
            .enumerate()
            .filter(|(i, _)| i % 2 == 0)
            .fold(0, |acc, n| acc + n.1)
}