pub fn get_single_number(mut input: Vec<i32>) -> i32 {
   input.sort();
   let mut unique = input[0];
   let mut is_unique = 1;

   for number in input.into_iter().skip(1) {

      if unique == number {
         is_unique += 1;
      } else {
         if is_unique == 1 {
            return unique;
         }
         is_unique = 1;
      }

      unique = number;
   }

   unique
}
