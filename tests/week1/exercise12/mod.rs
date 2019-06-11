use onboarding_rust::week1::exercise12::palindrome;

#[test]
fn test_week1_exercise12_example1() {

    let input = String::from("A man, a plan, a canal: Panama");
    let expected = true;
    let result = palindrome(input);
    assert_eq!(expected, result);
}

#[test]
fn test_week1_exercise12_example2() {

    let input = "race a car".to_string();
    let expected = false;
    let result = palindrome(input);
    assert_eq!(expected, result);
}
