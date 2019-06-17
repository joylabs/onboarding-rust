use onboarding_rust::week1::exercise12::is_palindrome;

#[test]
fn test_week1_exercise12_example1() {

    let input = "A man, a plan, a canal: Panama";
    let expected = true;
    assert_eq!(expected, is_palindrome(input));
}

#[test]
fn test_week1_exercise12_example2() {

    let input = "race a car";
    let expected = false;
    assert_eq!(expected, is_palindrome(input));
}