use onboarding_rust::week1::exercise48::find_min_arrow_shots;

#[test]
fn test_week1_exercise48_two_arrows() {
    let input = vec![vec![10, 16], vec![2, 8], vec![1, 6], vec![7, 12]];
    let expected = 2;
    assert_eq!(expected, find_min_arrow_shots(input));
}

#[test]
fn test_week1_exercise48_empty_array() {
    let input = vec![];
    let expected = 0;
    assert_eq!(expected, find_min_arrow_shots(input));
}

#[test]
fn test_week1_exercise48_mutiple_intersections() {
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
fn test_week1_exercise48_three_arrows() {
    let input = vec![vec![0, 2], vec![3, 5], vec![6, 8]];
    let expected = 3;
    assert_eq!(expected, find_min_arrow_shots(input));
}