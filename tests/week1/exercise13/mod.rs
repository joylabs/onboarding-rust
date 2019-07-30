use onboarding_rust::week1::exercise13::reverse_vowels;

#[test]
fn test_week1_exercise13_two_vowels() {
    let input: String = String::from("hello");
    let output: String = String::from("holle");
    assert_eq!(output, reverse_vowels(input));
}

#[test]
fn test_week1_exercise13_four_vowels() {
    let input: String = String::from("leetcode");
    let output: String = String::from("leotcede");
    assert_eq!(output, reverse_vowels(input));
}

#[test]
fn test_week1_exercise13_no_vowels() {
    let input: String = String::from("xyz");
    let output: String = String::from("xyz");
    assert_eq!(output, reverse_vowels(input));
}