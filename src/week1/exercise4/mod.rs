pub fn self_dividing_numbers(input: Vec<i32>) -> Vec<i32> {
   let mut output: Vec<i32> = vec![];
   const DIV: i32 = 10;

   for x in input[0]..=input[1] {
      let mut num = x;
      let mut is_div = true;

      println!("x: {}", x);

      while num >= DIV {

         let my_num = num % DIV;
         println!("num: {}", my_num);

         if my_num == 0 {
            is_div = false;
            break;
         } else if x % my_num == 0 {
            is_div = true;
         } else {
            is_div = false;
         }

         num /= DIV;
      }

      println!("final num: {}", num);

      if is_div {
         if num == 0 {
            is_div = false;
         } else if x % num == 0 {
            is_div = true;
         } else {
            is_div = false;
         }
      }


      println!("is_div: {}", is_div);

      num = x;
      if is_div {
         output.push(num);
      }
   }
   println!("output: {:?}", output);
   output
}