pub fn plus_one(input: Vec<i8>) -> Vec<i8> {
   let mut carry_on = true;
   let mut output: Vec<i8> = input
      .iter()
      .rev()
      .map(|x| {
         if carry_on {
            if *x < 9 {
               carry_on = false;
               return x + 1;
            }
            return 0;
         }
         *x
      })
      .collect();
   if carry_on {
      output.push(1);
   }
   output.into_iter().rev().collect()
}