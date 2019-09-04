use onboarding_rust::week1::exercise21::find_circle_num;

#[test]
fn test_week1_exercise21_two() {
    let input = vec![vec![1, 1, 0], vec![1, 1, 0], vec![0, 0, 1]];
    let output = 2;
    assert_eq!(output, find_circle_num(input));
}

#[test]
fn test_week1_exercise21_one() {
    let input = vec![vec![1, 1, 0], vec![1, 1, 1], vec![0, 1, 1]];
    let output = 1;
    assert_eq!(output, find_circle_num(input));
}