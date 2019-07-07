use std::collections::HashSet;

pub fn distribute_candies(input: Vec<i32>) -> i32 {
   let cadies_per_person = (input.len() / 2) as i32;
   let cady_kinds = input.into_iter().collect::<HashSet<i32>>().len() as i32;

   if cady_kinds >= cadies_per_person {
      cadies_per_person
   } else {
      cady_kinds
   }
}
