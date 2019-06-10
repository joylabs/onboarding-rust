use onboarding_rust::week1::exercise2::exercise2;

#[test]
fn test_week1_exercise2_example1() {
    // Input: [[1,2,3],[4,5,6],[7,8,9]]
    // Output: [[1,4,7],[2,5,8],[3,6,9]]
    let input = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    let expected = vec![vec![1, 4, 7], vec![2, 5, 8], vec![3, 6, 9]];
    assert_eq!(expected, exercise2(input));
}

#[test]
fn test_week1_exercise2_example2() {
    // Input: [[1,2,3],[4,5,6]]
    // Output: [[1,4],[2,5],[3,6]]
    let input = vec![vec![1, 2, 3], vec![4, 5, 6]];
    let expected = vec![vec![1, 4], vec![2, 5], vec![3, 6]];
    assert_eq!(expected, exercise2(input));
}