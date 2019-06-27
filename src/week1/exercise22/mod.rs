pub fn count_islands(input: Vec<Vec<char>>) -> i32 {
   if input.is_empty() {
      return 0;
   }

   let mut past_element_horizontal = '0';
   let mut past_element_vertical = '0';
   let mut next_element_horizontal;
   let mut next_element_vertical;
   let mut island_count = 0;

   for (i, x) in input.iter().enumerate() {
      for (j, y) in x.iter().enumerate() {

         if i > 0 {
            past_element_vertical = input[i - 1][j];
         }

         if j > 0 {
            past_element_horizontal = input[i][j - 1];
         }

         if input.len() > (i + 1) {
            next_element_vertical = input[i + 1][j];
         } else {
            next_element_vertical = '0';
         }

         if x.len() > (j + 1) {
            next_element_horizontal = input[i][j + 1];
         } else {
            next_element_horizontal = '0';
         }

         if *y == '1' && past_element_horizontal == '0' && past_element_vertical == '0'
         //&& next_element_horizontal == '0'
         //&& next_element_vertical == '0'
         {
            island_count += 1;
            println!(
               "p_v: {}, p_h: {}, n_v: {}, n_h: {}, (i: {}, j: {})",
               past_element_vertical,
               past_element_horizontal,
               next_element_vertical,
               next_element_horizontal,
               i,
               j
            );
         }
      }
   }

   island_count
}