use onboarding_rust::week1::exercise4::self_dividing_numbers;

#[test]
fn test_week1_exercise4_example1() {
    let input = vec![1,22];
    let expected = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 11, 12, 15, 22];
    assert_eq!(expected, self_dividing_numbers(input));
}
