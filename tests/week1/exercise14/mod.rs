use onboarding_rust::week1::exercise14::longest_common_prefix;
#[test]
fn test_week1_exercise14_example1() {
    let input = vec![
        String::from("flower"),
        String::from("flow"),
        String::from("flight"),
    ];
    let expected = String::from("fl");
    assert_eq!(expected, longest_common_prefix(input));
}
#[test]
fn test_week1_exercise14_example2() {
    let input = vec![
        String::from("dog"),
        String::from("racecar"),
        String::from("car"),
    ];
    let expected = String::from("");
    assert_eq!(expected, longest_common_prefix(input));
}