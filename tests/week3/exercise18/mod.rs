use onboarding_rust::week3::exercise18::rob;

#[test]
fn test_week3_exercise18_example1() {
    let input = vec![2, 3, 2];
    let expected = 3;
    assert_eq!(expected, rob(input));
}

#[test]
fn test_week3_exercise18_example2() {
    let input = vec![1, 2, 3, 1];
    let expected = 4;
    assert_eq!(expected, rob(input));
}

#[test]
fn test_week3_exercise18_example3() {
    let input = vec![];
    let expected = 0;
    assert_eq!(expected, rob(input));
}

#[test]
fn test_week3_exercise18_example4() {
    let input = vec![1];
    let expected = 1;
    assert_eq!(expected, rob(input));
}
