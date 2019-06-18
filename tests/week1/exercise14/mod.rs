use onboarding_rust::week1::exercise14::find_common_prefix;

#[test]
fn test_week1_exercise10_example1() {
    let input = vec!["flower","flow","flight"];
    let expected = "fl";
    assert_eq!(expected, find_common_prefix(input));
}

#[test]
fn test_week1_exercise10_example2() {
    let input = vec!["dog","racecar","car"];
    let expected = "";
    assert_eq!(expected, find_common_prefix(input));
}