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
   first_item
      .iter()
      .enumerate()
      .fold("".to_string(), |mut acc, (i, c)| {
         if (*c == last_item[i] ) && (acc.len() == i) {
            acc.push_str(&c.to_string());
         }
         acc
      })
}