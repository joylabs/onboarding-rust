use onboarding_rust::week1::exercise6::plus_one;

#[test]
fn test_week1_exercise6_example1() {
    let input = vec![1, 2, 3];
    let expected = vec![1, 2, 4];
    assert_eq!(expected, plus_one(input));
}
#[test]
fn test_week1_exercise6_example2() {
    let input = vec![4, 3, 2, 1];
    let expected = vec![4, 3, 2, 2];
    assert_eq!(expected, plus_one(input));
}

#[test]
fn test_week1_exercise6_example3() {
    let input = vec![4, 3, 2, 9];
    let expected = vec![4, 3, 3, 0];
    assert_eq!(expected, plus_one(input));
}

#[test]
fn test_week1_exercise6_example4() {
    let input = vec![4, 3, 9, 9];
    let expected = vec![4, 4, 0, 0];
    assert_eq!(expected, plus_one(input));
}

#[test]
fn test_week1_exercise6_example5() {
    let input = vec![9, 9, 9, 9];
    let expected = vec![1, 0, 0, 0, 0];
    assert_eq!(expected, plus_one(input));
}