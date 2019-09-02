use onboarding_rust::week1::exercise19::two_sum;

#[test]
fn test_week1_exercise19_nine() {
    let nums = vec![2, 7, 11, 15];
    let target = 9;
    let output = vec![0, 1];
    assert_eq!(output, two_sum(nums, target));
}

#[test]
fn test_week1_exercise19_six() {
    let nums = vec![3, 2, 4];
    let target = 6;
    let output = vec![1, 2];
    assert_eq!(output, two_sum(nums, target));
}

#[test]
fn test_week1_exercise19_eight() {
    let nums = vec![4, 4];
    let target = 8;
    let output = vec![0, 1];
    assert_eq!(output, two_sum(nums, target));
}