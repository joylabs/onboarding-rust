use onboarding_rust::week2::exercise40::find_error_nums;

#[test]
fn one_find_error() {
    let input = vec![1, 2, 2, 4];
    let output = vec![2, 3];
    assert_eq!(output, find_error_nums(input));
}

#[test]
fn two_find_error() {
    let input = vec![1, 1];
    let output = vec![1, 2];
    assert_eq!(output, find_error_nums(input));
}

#[test]
fn three_find_error() {
    let input = vec![1, 3, 3];
    let output = vec![3, 2];
    assert_eq!(output, find_error_nums(input));
}
