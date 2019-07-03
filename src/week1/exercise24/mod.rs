pub fn count_jewels(j: String, s: String) -> i32 {
   s.chars().fold(0, |mut acc, c| {
      if j.contains(c) {
         acc += 1;
      }
      acc
   })
}