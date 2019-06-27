pub fn get_two_sum(input: Vec<i32>, target: i32) -> Vec<i32> {
   let mut index_result: Vec<i32> = vec![];

   for (i, x) in input.iter().enumerate() {
      for (j, y) in input.iter().enumerate().skip(i + 1) {

         if (x + y) == target {
            index_result.push(i as i32);
            index_result.push(j as i32);
            break;
         }

      }
   }

   index_result
}