use onboarding_rust::week1::exercise14::find_common_prefix;

#[test]
fn test_week1_exercise14_example1() {
    let input = vec!["flower", "flow", "flight"];
    let expected = "fl".to_string();
    assert_eq!(expected, find_common_prefix(input));
}

#[test]
fn test_week1_exercise14_example2() {
    let input = vec!["dog", "racecar", "car"];
    let expected = "".to_string();
    assert_eq!(expected, find_common_prefix(input));
}

#[test]
fn test_week1_exercise14_example3() {
    let input: Vec<&str> = vec![];
    let expected = "".to_string();
    assert_eq!(expected, find_common_prefix(input));
}