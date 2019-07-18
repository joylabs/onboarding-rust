use onboarding_rust::week2::exercise6::get_group_eq_strings;

#[test]
fn test_week1_exercise9_example1() {
    let input = vec!["a", "b", "c", "a", "c", "c"];
    let expected = 3;
    assert_eq!(expected, get_group_eq_strings(input));
}