use std::collections::HashSet;

pub fn distribute_candies(input: Vec<i32>) -> i32 {
   let candies_per_person = (input.len() / 2) as i32;
   let candy_kinds = input.into_iter().collect::<HashSet<i32>>().len() as i32;

   if candy_kinds >= candies_per_person {
      candies_per_person
   } else {
      candy_kinds
   }
}
