use onboarding_rust::week1::exercise6::plus_one;

#[test]
fn test_week1_exercise6_increment_first() {
    let input = vec![1, 2, 3];
    let output = vec![1, 2, 4];
    assert_eq!(output, plus_one(input));
}

#[test]
fn test_week1_exercise6_carry_over_once() {
    let input = vec![1, 2, 3, 9];
    let output = vec![1, 2, 4, 0];
    assert_eq!(output, plus_one(input));
}

#[test]
fn test_week1_exercise6_carry_over_twice() {
    let input = vec![1, 2, 3, 9, 9];
    let output = vec![1, 2, 4, 0, 0];
    assert_eq!(output, plus_one(input));
}

#[test]
fn test_week1_exercise6_carry_over_last_number() {
    let input = vec![9];
    let output = vec![1, 0];
    assert_eq!(output, plus_one(input));
}

#[test]
fn test_week1_exercise6_carry_over_last_number_multiple() {
    let input = vec![9, 9, 9, 9];
    let output = vec![1, 0, 0, 0, 0];
    assert_eq!(output, plus_one(input));
}