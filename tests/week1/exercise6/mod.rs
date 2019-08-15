use onboarding_rust::week1::exercise6::plus_one;

#[test]
fn test_week1_exercise6_three_digits() {
    let input = vec![1, 2, 3];
    let output = vec![1, 2, 4];
    assert_eq!(output, plus_one(input));
}

#[test]
fn test_week1_exercise6_four_digits() {
    let input = vec![4, 3, 2, 1];
    let output = vec![4, 3, 2, 2];
    assert_eq!(output, plus_one(input));
}
#[test]
fn test_week1_exercise6_two_digits() {
    let input = vec![9];
    let output = vec![1, 0];
    assert_eq!(output, plus_one(input));
}