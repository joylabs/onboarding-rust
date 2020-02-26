use onboarding_rust::week2::exercise7::intersection;

#[test]
fn test_week2_exercise8_example1() {
    let input_a = vec![1, 2, 2, 1];
    let input_b = vec![2, 2];

    let expected = vec![2];

    assert_eq!(expected, intersection(input_a, input_b));
}

#[test]
fn test_week2_exercise8_example2() {
    let input_a = vec![4, 9, 5];
    let input_b = vec![9, 4, 9, 8, 4];

    let expected = vec![9, 4];

    assert_eq!(expected, intersection(input_a, input_b));
}
