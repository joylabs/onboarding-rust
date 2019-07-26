use onboarding_rust::week2::exercise19::find_error_nums;

#[test]
fn test_week2_exercise19_example1() {
    let input = vec![1, 2, 2, 4];
    let expected = vec![2, 3];
    assert_eq!(expected, find_error_nums(input));
}

#[test]
fn test_week2_exercise19_example2() {
    let input = vec![1, 1];
    let expected = vec![1, 2];
    assert_eq!(expected, find_error_nums(input));
}

#[test]
fn test_week2_exercise19_example3() {
    let input = vec![1, 5, 3, 2, 2, 7, 6, 4, 8, 9];
    let expected = vec![2, 10];
    assert_eq!(expected, find_error_nums(input));
}
