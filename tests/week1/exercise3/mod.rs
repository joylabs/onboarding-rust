use onboarding_rust::week1::exercise3::matrix_flip_inverse;

#[test]
fn test_week1_exercise3_example1() {
    let input = vec![vec![1, 1, 0], vec![1, 0, 1], vec![0, 0, 0]];
    let expected = vec![vec![1, 0, 0], vec![0, 1, 0], vec![1, 1, 1]];
    assert_eq!(expected, matrix_flip_inverse(input));
}

#[test]
fn test_week1_exercise3_example2() {
    let input = vec![
        vec![1, 1, 0, 0],
        vec![1, 0, 0, 1],
        vec![0, 1, 1, 1],
        vec![1, 0, 1, 0],
    ];
    let expected = vec![
        vec![1, 1, 0, 0],
        vec![0, 1, 1, 0],
        vec![0, 0, 0, 1],
        vec![1, 0, 1, 0],
    ];
    assert_eq!(expected, matrix_flip_inverse(input));
}
