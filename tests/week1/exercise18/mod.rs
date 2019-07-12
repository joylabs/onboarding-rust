use onboarding_rust::week1::exercise18::single_number;

#[test]
fn test_week1_exercise18_example1() {
    let input = vec![2, 2, 1];
    let expected = 1;
    assert_eq!(expected, single_number(input));
}
#[test]
fn test_week1_exercise18_example2() {
    let input = vec![1, 4, 2, 1, 2];
    let expected = 4;
    assert_eq!(expected, single_number(input));
}

#[test]
fn test_week1_exercise18_example3() {
    let input = vec![1, 4, 2, 3, 4, 1, 2];
    let expected = 3;
    assert_eq!(expected, single_number(input));
}