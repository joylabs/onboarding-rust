pub fn partition_labels(s: String) -> Vec<i32> {
      let mut result: Vec<i32> = Vec::new();
      let mut indexes: Vec<Vec<i32>> = Vec::new();
      let mut letters: Vec<char> = Vec::new();
      
      s.chars().for_each(|c|{
            if !letters.contains(&c) {
                  letters.push(c);
            }
      });

      letters.iter().for_each(|c| {
            
      });
      
      result
}

fn find_last_index(letter: char, word: &str) -> i32 {
      word.chars().enumerate().fold(0, |acc, (i, c)| {
            if letter == c {
                  i as i32 + 1
            } else {
                  acc
            }
      })
}

fn is_greater(index: i32, nums: &[i32]) -> bool {
      for n in nums.iter() {
            if *n > index {
                  return false;
            }
      }
      true
}