pub fn get_group_eq_strings(input: Vec<&str>) -> i32 {
   let mut strings: Vec<String> = input.iter().map(|w| w.to_string()).collect();
   
   if input[0].len() > 2 {
      strings = strings.iter_mut().map(|word| sort_string(word)).collect();
   }

   strings.sort();
   strings.dedup(); 

   strings.len() as i32
}

fn sort_string(str1: &str) -> String {
   let mut odd: Vec<char> = str1.chars().enumerate().filter(|c| c.0 % 2 == 1).map(|c| c.1).collect();
   let mut even: Vec<char> = str1.chars().enumerate().filter(|c| c.0 % 2 == 0).map(|c| c.1).collect();
   odd.sort();
   even.sort();
   odd.iter().collect::<String>() + &even.iter().collect::<String>()
}