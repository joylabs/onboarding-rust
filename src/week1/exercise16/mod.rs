pub fn get_hamming_distance(input: (u32, u32)) -> u32 {
   let num_difference = input.0 ^ input.1;
   let num_string = format!("{:b}", num_difference);

   num_string.chars().fold(0, |mut acc, n| {
      if n == '1' {
         acc += 1;
      }
      acc
   })
}
