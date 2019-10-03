use onboarding_rust::week2::exercise5::distribute_candies;

#[test]
fn test_week2_exercise5_3() {
    let input = vec![1, 1, 2, 2, 3, 3];
    let expected = 3;
    assert_eq!(expected, distribute_candies(input));
}

#[test]
fn test_week2_exercise5_2() {
    let input = vec![1, 1, 2, 3];
    let expected = 2;
    assert_eq!(expected, distribute_candies(input));
}

#[test]
fn test_week2_exercise5_1() {
    let input = vec![1, 1, 1, 1];
    let expected = 1;
    assert_eq!(expected, distribute_candies(input));
}
