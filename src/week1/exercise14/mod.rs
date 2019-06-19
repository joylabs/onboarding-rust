pub fn find_common_prefix(mut input: Vec<&str>) -> String {
   if input.is_empty() {
      return "".to_string();
   }

   input.sort();

   let first_item = input[0].chars().collect::<Vec<char>>();
   let last_item = input[input.len() - 1].chars().collect::<Vec<char>>();

   get_common_prefix(first_item, last_item)
}

fn get_common_prefix(first_item: Vec<char>, mut last_item: Vec<char>) -> String {
   first_item
      .iter()
      .enumerate()
      .fold("".to_string(), |mut acc, (i, c)| {
         if !last_item.is_empty() {
            if *c == last_item[i] {
               acc.push_str(&c.to_string());
            } else {
               last_item = vec![];
            }
         }
         acc
      })
}