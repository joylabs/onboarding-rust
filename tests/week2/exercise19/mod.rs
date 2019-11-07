use onboarding_rust::week2::exercise19::find_error_nums;

#[test]
fn test_week2_exercise18_three() {
    let input = vec![1, 2, 2, 4];
    let result = vec![2, 3];
    assert_eq!(result, find_error_nums(input));
}

#[test]
fn test_week2_exercise18_one() {
    let input = vec![3, 2, 3, 4, 6, 5];
    let result = vec![3, 1];
    assert_eq!(result, find_error_nums(input));
}
