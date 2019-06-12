pub fn get_column_number(input: &str) -> i32 {
   const OFFSET: i32 = ('A' as i32) - 1;
   const BASE: i32 = 26;

   input
      .chars()
      .rev()
      .enumerate()
      .map(|(i, c)| {
         let base_26 = (c as i32) - OFFSET;
         base_26 * BASE.pow(i as u32)
      })
      .sum()

}