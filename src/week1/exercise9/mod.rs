pub fn reverse_string(mut input: Vec<char>) -> Vec<char> {
   let mut letter;
   let mut last_index = input.len() - 1;

   for i in 0..input.len() {
      if last_index > i {
         letter = input[i];
         input[i] = input[last_index];
         input[last_index] = letter;
         last_index -= 1;
      } else {
         break;
      }
   }

   println!("{:?}", input);

   input
}