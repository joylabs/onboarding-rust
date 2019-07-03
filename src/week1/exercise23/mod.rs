pub fn capture_surrounded_regions(input: &mut Vec<Vec<char>>) {
   if !input.is_empty() && (input.len() > 2) && (input[0].len() > 2) {
      let last_row_index = input.len() - 1;
      let last_element_index = input[0].len() - 1;
      let input_clone = input.clone();

      for (i, row) in input.iter_mut().enumerate().skip(1) {
         for (j, element) in row.iter_mut().enumerate().skip(1) {
            if i < last_row_index && j < last_element_index {
               let is_surrounded =
                  is_surrounded(i, j, last_row_index, last_element_index, &input_clone);

               if is_surrounded {
                  *element = 'X';
               }
            }
         }
      }
   }
}

fn is_surrounded(
   i: usize,
   j: usize,
   last_row_index: usize,
   last_element_index: usize,
   board: &[Vec<char>],
) -> bool {
   let element_index = last_element_index - 1;
   let row_index = last_row_index - 1;

   if i == 1 {

      if j == 1 {
         board[i][0] == 'X' && board[0][j] == 'X'
      } else if j == element_index {
         board[i][last_element_index] == 'X' && board[0][j] == 'X'
      } else {
         board[0][j] == 'X'
      }

   } else if i == row_index {

      if j == 1 {
         board[i][0] == 'X' && board[last_row_index][j] == 'X'
      } else if j == element_index {
         board[i][last_element_index] == 'X' && board[last_row_index][j] == 'X'
      } else {
         board[last_row_index][j] == 'X'
      }

   } else if j == 1 {
      board[i][0] == 'X'
   } else if j == element_index {
      board[i][last_element_index] == 'X'
   } else {
      false
   }
}
