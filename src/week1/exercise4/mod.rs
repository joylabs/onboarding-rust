pub fn self_dividing_numbers(input: Vec<i32>) -> Vec<i32> {
   let mut output: Vec<i32> = vec![];

   for x in input[0]..=input[1] {
      let is_div = is_self_dividing_number(x);
      println!("is_div: {}", is_div);

      if is_div {
         output.push(x);
      }
   }

   println!("output: {:?}", output);
   output
}

pub fn is_self_dividing_number(n: i32) -> bool {
   const DIV: i32 = 10;
   let mut num = n;

   while num >= DIV {

      let my_num = num % DIV;
      println!("num: {}", my_num);

      if my_num == 0 {
         return false;
      }
      
      if n % my_num != 0 {
         return false;
      }

      num /= DIV;
   }

   println!("final num: {}", num);

   if num == 0 {
      return false;
   }
   
   if n % num != 0 {
      false
   } else {
      true
   }
}