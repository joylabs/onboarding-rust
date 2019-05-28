use onboarding_rust::week1::exercise2::exercise_two;

#[test]
fn test_week1_exercise1() {

    ///[[1,4,7],[2,5,8],[3,6,9]]
    let first = vec![1,4,7];
    let second = vec![2,5,8];
    let third = vec![3,6,9];
    let output = vec![first, second, third];
    let input = vec![vec![1,2,3],vec![4,5,6],vec![7,8,9]];

    assert_eq!(output, exercise_two(input));
}
