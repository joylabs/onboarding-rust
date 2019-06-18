pub fn get_complement_number(input: i32) -> i32 {
   if input == 0 {
      1
   } else if input > 0 {
      input >> 1
   } else {
      !input
   }
}