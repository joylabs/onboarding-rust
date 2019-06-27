pub fn binary_gap(input: u32) -> u32 {
   if input.count_ones() == 1 {
      return 0;
   }

   let number_in_string = format!("{:b}", input);
   let mut longest_distance = 1;
   let mut acc = 1;

   for bit in number_in_string.chars() {
      if bit == '0' {
         acc += 1;
      } else if bit == '1' {
         if acc > longest_distance {
            longest_distance = acc;
         }
         acc = 1;
      }
   }

   longest_distance
}