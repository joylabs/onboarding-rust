pub fn is_anagram(input: Vec<&str>) -> bool {

   if input[0].len() != input[1].len() {
      return false;
   }

   let mut word_1: Vec<char> = input[0].chars().collect();
   let mut word_2: Vec<char> = input[1].chars().collect();

   word_1.sort();
   word_2.sort();

   word_1 == word_2
}