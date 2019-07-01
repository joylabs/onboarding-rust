use onboarding_rust::week1::exercise17::binary_gap;

#[test]
fn test_week1_exercise17_example1() {
    let input = 22;
    let expected = 2;
    assert_eq!(expected, binary_gap(input));
}
#[test]
fn test_week1_exercise17_example2() {
    let input = 5;
    let expected = 2;
    assert_eq!(expected, binary_gap(input));
}
#[test]
fn test_week1_exercise17_example3() {
    let input = 6;
    let expected = 1;
    assert_eq!(expected, binary_gap(input));
}
#[test]
fn test_week1_exercise17_example4() {
    let input = 8;
    let expected = 0;
    assert_eq!(expected, binary_gap(input));
}