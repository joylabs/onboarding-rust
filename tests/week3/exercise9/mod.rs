use onboarding_rust::week3::exercise9::merge;

#[test]
fn test_week3_exercise9_example1() {
    let input = vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]];
    let expected = vec![vec![1, 6], vec![8, 10], vec![15, 18]];
    assert_eq!(expected, merge(input));
}

#[test]
fn test_week3_exercise9_example2() {
    let input = vec![vec![1, 4], vec![4, 5]];
    let expected = vec![vec![1, 5]];;
    assert_eq!(expected, merge(input));
}