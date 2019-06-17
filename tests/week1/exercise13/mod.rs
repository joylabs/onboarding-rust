use onboarding_rust::week1::exercise13::reverse_vowels;

#[test]
fn test_week1_exercise10_example1() {
    let input = "hello";
    let expected = "holle".to_string();
    assert_eq!(expected, reverse_vowels(input));
}

#[test]
fn test_week1_exercise10_example2() {
    let input = "leetcode";
    let expected = "leotcede".to_string();
    assert_eq!(expected, reverse_vowels(input));
}