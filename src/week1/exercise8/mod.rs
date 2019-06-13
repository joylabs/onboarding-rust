pub fn is_power_of_two(input: i32) -> bool {
   let mut result = input;

   while result % 2 == 0 {
      result /= 2;
   }

   result == 1
}