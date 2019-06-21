pub fn get_complement_number(input: i32) -> i32 {
   is_valid_number(input);

   let binary_number_string = format!("{:b}",input);
   let leading_zeros = 32 - binary_number_string.len();

   (!input) << leading_zeros >> leading_zeros
}

fn is_valid_number(input: i32) {
   if input < 1 {
      panic!("Number must be a positive and greater than 0")
   }
}