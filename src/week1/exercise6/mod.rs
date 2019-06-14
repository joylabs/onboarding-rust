pub fn plus_one(input: Vec<i8>) -> Vec<i8> {
   let mut output = input.clone();
   let mut carry_on = true;

   for i in output.iter_mut().rev() {
      if carry_on {
         if *i < 9 {
            *i += 1;
            carry_on = false;
         } else {
            *i = 0;
            carry_on = true;
         }
      }
   }

   if carry_on {
      let mut o: Vec<i8> = vec![1];
      o.append(&mut output);
      output = o;
   }

   output
}