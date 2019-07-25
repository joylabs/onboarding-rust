use onboarding_rust::week1::exercise12::is_palindrome;

#[test]
fn test_week1_exercise12_example1() {

    let input = String::from("A man, a plan, a canal: Panama");
    let expected = true;

    assert_eq!(expected, is_palindrome(input));
}

#[test]
fn test_week1_exercise12_example4() {

    let input = String::from("0P");
    let expected = false;

    assert_eq!(expected, is_palindrome(input));
}

#[test]
fn test_week1_exercise12_example2() {

    let input = "race a car".to_string();
    let expected = false;

    assert_eq!(expected, is_palindrome(input));
}