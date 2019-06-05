use onboarding_rust::week1::exercise1::order_numbers;

#[test]
fn test_week1_exercise1() {
    assert_eq!(vec![2, 4, 3, 1], order_numbers(vec![3, 1, 2, 4]));
}
