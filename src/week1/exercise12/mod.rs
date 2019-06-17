pub fn is_palindrome(input: &str) -> bool {
   let original = input
      .chars()
      .filter(|c| c.is_alphanumeric())
      .collect::<String>()
      .to_lowercase();
   let reversed_original = original.chars().rev().collect::<String>();

   original == reversed_original
}