use onboarding_rust::week1::exercise12::is_palindrome;

#[test]
fn test_week1_exercise12_panama() {
    let input = String::from("A man, a plan, a canal: Panama");
    let output = true;
    assert_eq!(output, is_palindrome(input));
}

#[test]
fn test_week1_exercise12_car() {
    let input = String::from("race a car");
    let output = false;
    assert_eq!(output, is_palindrome(input));
}

#[test]
fn test_week1_exercise12_whitespace() {
    let input = String::from(" ");
    let output = true;
    assert_eq!(output, is_palindrome(input));
}

#[test]
fn test_week1_exercise12_number() {
    let input = String::from("1221");
    let output = true;
    assert_eq!(output, is_palindrome(input));
}