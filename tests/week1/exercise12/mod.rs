use onboarding_rust::week1::exercise12::valid_palindrome;

#[test]
fn test_week1_exercise12_valid_palindrome() {
    let input: String = String::from("A man, a plan, a canal: Panama");
    let output: bool = true;
    assert_eq!(output, valid_palindrome(input));
}

#[test]
fn test_week1_exercise12_not_valid() {
    let input: String = String::from("race a car");
    let output: bool = false;
    assert_eq!(output, valid_palindrome(input));
}

#[test]
fn test_week1_exercise12_empty_string() {
    let input: String = String::from("");
    let output: bool = true;
    assert_eq!(output, valid_palindrome(input));
}