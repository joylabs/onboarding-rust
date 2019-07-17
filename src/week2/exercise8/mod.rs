use std::collections::HashSet;

pub fn is_valid_sudoku(input: Vec<Vec<char>>) -> bool {
   for i in 0..9 {
      let mut column = Vec::new();

      for row in input.iter() {
         if !is_valid_line(row) {
            return false;
         }
         column.push(row[i])
      }
      // println!("column {:?}", column);

      if !is_valid_line(&column) {
         return false;
      }
   }

   true
}

fn is_valid_line(input: &Vec<char>) -> bool {
   let total_elements = 
      input
         .iter()
         .fold(0, |mut acc, x| {
            if *x != '.' {
               acc += 1;
            }
            acc
         });

   let unique_elements: HashSet<char> = 
      input
         .iter()
         .filter_map(|x| {
            if *x != '.' {
               Some(*x)
            } else {
               None
            }
         })
         .collect();

   // println!("total: {} unique: {}", total_elements, unique_elements.len());

   total_elements == unique_elements.len()
}

// fn eval_sub_boxes(input: Vec<Vec<char>>) -> bool {
//    for x in 0..9 {
//       let mut range_x = 2;
//       let mut range_y = 2;
//       let mut sub_box = Vec::new();

//       for (i, row) in input.iter().enumerate() {
//          sub_box.chunks(chunk_size: usize)
//       }


//       range += 3;
//    }

//    true
// }
