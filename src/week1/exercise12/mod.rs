pub fn is_palindrome(input: String) -> bool {
    let original = input
        .chars()
        .filter(|c| c.is_alphanumeric())
        .collect::<String>()
        .to_lowercase();
    let reversed = original.chars().rev().collect::<String>();

    original == reversed
}