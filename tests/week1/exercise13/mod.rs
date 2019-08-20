use onboarding_rust::week1::exercise13::reverse_vowels;

#[test]
fn test_week1_exercise13_hello() {
    let input = String::from("Hello");
    let output = String::from("Holle");
    assert_eq!(output, reverse_vowels(input));
}

#[test]
fn test_week1_exercise13_leetcode() {
    let input = String::from("leetcode");
    let output = String::from("leotcede");
    assert_eq!(output, reverse_vowels(input));
}

#[test]
fn test_week1_exercise13_aa() {
    let input = String::from("aA");
    let output = String::from("Aa");
    assert_eq!(output, reverse_vowels(input));
}

#[test]
fn test_week1_exercise13_whitespace() {
    let input = String::from(" ");
    let output = String::from(" ");
    assert_eq!(output, reverse_vowels(input));
}