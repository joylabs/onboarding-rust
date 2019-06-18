use onboarding_rust::week1::exercise13::reverse_vowels_string;

#[test]
fn test_week1_exercise13_example1() {

    let input = "hello";
    let expected = "holle";
    assert_eq!(expected, reverse_vowels_string(input));
}

#[test]
fn test_week1_exercise13_example2() {

    let input = "leetcode";
    let expected = "leotcede";
    assert_eq!(expected, reverse_vowels_string(input));
}
