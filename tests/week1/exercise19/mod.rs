use onboarding_rust::week1::exercise19::get_two_sum;

#[test]
fn test_week1_exercise16_example1() {
    let input = vec![2, 7, 11, 15];
    let target = 9;
    let expected = vec![0, 1];
    assert_eq!(expected, get_two_sum(input, target));
}