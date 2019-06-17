use onboarding_rust::week1::exercise20::is_anagram;

#[test]
fn test_week1_exercise20_example1() {
    let input = vec!["anagram", "nagaram"];
    let expected = true;
    assert_eq!(expected, is_anagram(input));
}

#[test]
fn test_week1_exercise20_example2() {
    let input = vec!["rat", "car"];
    let expected = false;
    assert_eq!(expected, is_anagram(input));
}