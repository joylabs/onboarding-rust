pub fn detect_capital(input: &str) -> bool {
   let all_uppercase = input.to_uppercase() == input;
   let all_lowercase = input.to_lowercase() == input;
   let first_is_uppercase =
      (input[0..1].to_uppercase() == input[0..1]) && (input[1..].to_lowercase() == input[1..]);

   all_uppercase || all_lowercase || first_is_uppercase
}