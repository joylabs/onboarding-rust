use onboarding_rust::week2::exercise16::find_the_difference;

#[test]
fn test_week2_exercise14_example1() {
    let s = "abcd".to_string();
    let t = "abcde".to_string();
    let output = 'e';
    assert_eq!(output, find_the_difference(s, t));
}
