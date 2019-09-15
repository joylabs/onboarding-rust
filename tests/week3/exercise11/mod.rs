use onboarding_rust::week3::exercise11::max_profit;

#[test]
fn test_week3_exercise11_example1() {
    let input = vec![7, 1, 5, 3, 6, 4];
    let expected = 5;
    assert_eq!(expected, max_profit(input));
}

#[test]
fn test_week3_exercise11_example2() {
    let input = vec![7, 6, 4, 3, 1];
    let expected = 0;
    assert_eq!(expected, max_profit(input));
}

#[test]
fn test_week3_exercise11_example3() {
    let input = vec![2, 4, 1];
    let expected = 2;
    assert_eq!(expected, max_profit(input));
}

#[test]
fn test_week3_exercise11_example4() {
    let input = vec![];
    let expected = 0;
    assert_eq!(expected, max_profit(input));
}
