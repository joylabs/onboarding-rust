pub fn get_complement_number(mut input: i32) -> i32 {
   const BIT_31: i32 = 1 << 30;

   if input == 0 {
      1
   } else if input > 0 {
      let mut shift_counter = 1;

      while input < BIT_31 {
         input <<= 1;
         shift_counter += 1;
      }
      input <<= 1;

      (!input) >> shift_counter
   } else {
      !input
   }
}