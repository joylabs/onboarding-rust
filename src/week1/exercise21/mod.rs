struct IndirectFriendFinder {
   index_vector: Vec<i32>,
   circle_count: i32,
}

impl IndirectFriendFinder {
   fn init_friend_finder(matrix_order: i32) -> IndirectFriendFinder {
      IndirectFriendFinder {
         index_vector: (0..matrix_order).map(|x| x).collect(),
         circle_count: matrix_order,
      }
   }

   fn find_friend(&mut self, mut position: usize) -> usize {
      let friend = self.index_vector[position] as usize;
      if position != friend {
         self.index_vector[position] = self.index_vector[position];
         position = friend;
      }
      position
   }
}

pub fn count_friend_circles(input: Vec<Vec<i32>>) -> i32 {
   let matrix_order = input.len() as i32;
   let mut indirect_friend_finder = IndirectFriendFinder::init_friend_finder(matrix_order);

   input
      .into_iter()
      .enumerate()
      .for_each(|(i, row)| {
         get_one_count(i, row, &mut indirect_friend_finder);
      });

   indirect_friend_finder.circle_count
}

fn get_one_count(i: usize, row: Vec<i32>, indirect_friend_finder: &mut IndirectFriendFinder) {
   row.into_iter()
      .enumerate()
      .for_each(|(j, element)| {
         println!("element {}", element);

         let a = indirect_friend_finder.find_friend(i);
         let b = indirect_friend_finder.find_friend(j);

         if element == 1 && a != b {
            indirect_friend_finder.index_vector[a] = b as i32;
            indirect_friend_finder.circle_count -= 1;
         }
      });
}