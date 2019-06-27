pub fn get_two_sum(input: Vec<i32>, target: i32) -> Vec<i32> {
   let mut index_result: Vec<i32> = vec![];
   let mut j = 0;
   let mut element = 0;

   input.into_iter().enumerate().for_each(|(i, x)| {
      if x < target {
         if (element + x) == target {
            index_result.push(j as i32);
            index_result.push(i as i32);
         } else {
            j = i;
            element = x;
         }
      }
   });

   index_result
}
