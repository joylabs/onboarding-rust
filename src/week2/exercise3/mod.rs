pub fn is_happy_number(input: i32) -> bool {
   let mut number = get_digit_sum(input);
   let mut iteration_count = 0;

   while (number != 1) && (iteration_count < 100) {
      number = get_digit_sum(number);
      iteration_count += 1;
   }

   number == 1
}

fn get_digit_sum(number: i32) -> i32 {
   let digit_string = number.to_string();

   digit_string.chars().fold(0, |acc, c| {
      let num = c.to_digit(10).unwrap() as i32;
      acc + num.pow(2)
   })

}
