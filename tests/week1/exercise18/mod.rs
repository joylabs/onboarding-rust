use onboarding_rust::week1::exercise18::single_number;

#[test]
fn test_week1_exercise18_one() {
    let input = vec![2, 2, 1];
    let output = 1;
    assert_eq!(output, single_number(input));
}

#[test]
fn test_week1_exercise18_zero() {
    let input = vec![1, 0, 1];
    let output = 0;
    assert_eq!(output, single_number(input));
}