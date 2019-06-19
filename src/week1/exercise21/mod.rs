pub fn is_friend_circle(input: Vec<Vec<i32>>) -> i32 {
   let matrix_order = input.len() as i32;

   input
      .into_iter()
      .enumerate()
      .fold(matrix_order, |mut friend_count, (i, row)| {

         let mut index_i = i;
         let mut index_j = i;

         let one_count = get_one_count(row);

         println!("one_count {}", one_count);

         if one_count == matrix_order {
            friend_count = 1;
         } else if (one_count > 1) && (friend_count > 1) {
            friend_count -= 1;
         }

         friend_count
      })
}

fn get_one_count(row: Vec<i32>) -> i32 {
   row.into_iter()
      .enumerate()
      .fold(0, |mut counter, (j, element)| {
         println!("element {}", element);

         if element == 1 {
            counter += 1;
         }

         counter
      })
}