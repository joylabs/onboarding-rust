use onboarding_rust::week1::exercise2::transpose_matrix;

#[test]
fn test_week1_exercise2() {
    let input = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    let output = vec![vec![1, 4, 7], vec![2, 5, 8], vec![3, 6, 9]];
    assert_eq!(output, transpose_matrix(input));
}

#[test]
fn test_week1_exercise2_2() {
    let input = vec![vec![1, 2, 3], vec![4, 5, 6]];
    let output = vec![vec![1, 4], vec![2, 5], vec![3, 6]];
    assert_eq!(output, transpose_matrix(input));
}
