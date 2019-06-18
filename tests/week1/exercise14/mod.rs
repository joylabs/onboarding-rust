use onboarding_rust::week1::exercise14::longest_common_prefix;

#[test]
fn test_week1_exercise14_example1() {
    let input = vec!["flower", "flow", "flight"];
    let expected = "fl";

    assert_eq!(expected, longest_common_prefix(input));
}


#[test]
fn test_week1_exercise14_example2() {
    let input = vec!["dog", "racecar", "car"];
    let expected = "";

    assert_eq!(expected, longest_common_prefix(input));
}
#[test]
fn test_week1_exercise14_example3() {
    let input = vec![];
    let expected = "";

    assert_eq!(expected, longest_common_prefix(input));
}