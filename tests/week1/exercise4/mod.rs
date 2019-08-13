use onboarding_rust::week1::exercise4::self_dividing_numbers;

#[test]
fn test_week1_exercise4_one_to_twentytwo() {
    let in_left = 1;
    let in_right = 22;
    let output = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 11, 12, 15, 22];
    assert_eq!(output, self_dividing_numbers(in_left, in_right));
}