use onboarding_rust::week3::exercise4::find_min_arrow_shots;

#[test]
fn test_week3_exercise4_example1() {
    let input = vec![vec![10, 16], vec![2, 8], vec![1, 6], vec![7, 12]];
    let expected = 2;
    assert_eq!(expected, find_min_arrow_shots(input));
}

#[test]
fn test_week3_exercise4_example2() {
    let input = vec![];
    let expected = 0;
    assert_eq!(expected, find_min_arrow_shots(input));
}

#[test]
fn test_week3_exercise4_example3() {
    let input = vec![
        vec![3, 9],
        vec![7, 12],
        vec![3, 8],
        vec![6, 8],
        vec![9, 10],
        vec![2, 9],
        vec![0, 9],
        vec![3, 9],
        vec![0, 6],
        vec![2, 8],
    ];
    let expected = 2;
    assert_eq!(expected, find_min_arrow_shots(input));
}

#[test]
fn test_week3_exercise4_example4() {
    let input = vec![vec![0, 2], vec![3, 5], vec![6, 8]];
    let expected = 3;
    assert_eq!(expected, find_min_arrow_shots(input));
}