use onboarding_rust::week1::exercise2::transpose_matrix;

#[test]
fn transpose_matrix_test() {
    assert_eq!(vec![vec![1,4,7],vec![2,5,8],vec![3,6,9]], transpose_matrix(vec![vec![1,2,3], vec![4,5,6],vec![7,8,9]]));
}


#[test]
fn transpose_matrix_test_2() {
    assert_eq!(vec![vec![1,4],vec![2,5],vec![3,6]], transpose_matrix(vec![vec![1,2,3], vec![4,5,6]]));
}