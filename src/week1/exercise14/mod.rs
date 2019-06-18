pub fn find_common_prefix(mut input: Vec<&str>) -> String {
   if input.is_empty() {
      return "".to_string();
   }

   input.sort();

   let first_item = input[0].chars().collect::<Vec<char>>();
   let last_item = input[input.len() - 1].chars().collect::<Vec<char>>();

   get_common_prefix(first_item, last_item)
}

fn get_common_prefix(first_item: Vec<char>, last_item: Vec<char>) -> String {
   let mut is_different = false;

   first_item
      .iter()
      .enumerate()
      .fold("".to_string(), |mut acc, (i, c)| {
         if !is_different {
            if *c == last_item[i] {
               acc.push_str(&c.to_string());
            } else {
               is_different = true;
            }
         }
         acc
      })
}