use onboarding_rust::week1::exercise21::find_circle_num;

#[test]
fn test_week1_exercise21_example1() {
    let input = vec![vec![1, 1, 0], vec![1, 1, 0], vec![0, 0, 1]];
    let expected = 2;
    assert_eq!(expected, find_circle_num(input));
}

#[test]
fn test_week1_exercise21_example2() {
    let input = vec![vec![1, 1, 0], vec![1, 1, 1], vec![0, 1, 1]];
    let expected = 1;
    assert_eq!(expected, find_circle_num(input));
}

#[test]
fn test_week1_exercise21_example3() {
    let input = vec![
        vec![1, 0, 0, 1],
        vec![0, 1, 1, 0],
        vec![0, 1, 1, 1],
        vec![1, 0, 1, 1],
    ];
    let expected = 1;
    assert_eq!(expected, find_circle_num(input));
}
