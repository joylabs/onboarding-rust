pub fn valid_palindrome(s: String) -> bool {
    let input = s
        .to_lowercase()
        .chars()
        .filter(|character| character.is_alphanumeric())
        .collect::<String>();
    let input_half = input.len() / 2;
    input
        .chars()
        .take(input_half)
        .eq(input.chars().rev().take(input_half))
}