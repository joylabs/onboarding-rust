use onboarding_rust::week1::exercise1::array_by_parity;

#[test]
fn test_week1_exercise1() {
    let input = vec![3, 1, 2, 4];
    let output = vec![2, 4, 3, 1];
    assert_eq!(output, array_by_parity(input));
}

#[test]
fn test_week1_exercise1_all_evens() {
    let input = vec![2, 4, 8, 16];
    let output = vec![2, 4, 8, 16];
    assert_eq!(output, array_by_parity(input));
}

#[test]
fn test_week1_exercise1_all_odds() {
    let input = vec![1, 3, 5, 7, 9];
    let output = vec![1, 3, 5, 7, 9];
    assert_eq!(output, array_by_parity(input));
}

#[test]
fn test_week1_exercise1_assorted() {
    let input = vec![3, 2, 4, 9, 8, 5];
    let output = vec![2, 4, 8, 3, 9, 5];
    assert_eq!(output, array_by_parity(input));
}