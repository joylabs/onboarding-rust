use onboarding_rust::week1::exercise19::two_sum;

#[test]
fn test_week1_exercise19_two_sum() {
    let nums = vec![2, 7, 11, 15];
    let target = 9;
    let output = vec![0, 1];
    assert_eq!(output, two_sum(nums, target));
}
