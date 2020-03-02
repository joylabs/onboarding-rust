use onboarding_rust::week2::exercise5::distribute_candies;

#[test]
fn test_week2_exercise5_example1() {
    let input_a = vec![1, 1, 2, 2, 3, 3];

    let expected = 3;

    assert_eq!(expected, distribute_candies(input_a));
}

#[test]
fn test_week2_exercise5_example2() {
    let input_a = vec![1, 1, 2, 3];

    let expected = 2;

    assert_eq!(expected, distribute_candies(input_a));
}
