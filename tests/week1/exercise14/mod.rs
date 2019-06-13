use onboarding_rust::week1::exercise14::hamming_distance;

#[test]
fn test_week1_exercise14_example1() {

    let input = vec![1, 4];
    let expected = 2;
    assert_eq!(expected, hamming_distance(input));
}
#[test]
fn test_week1_exercise14_example2() {

    let input = vec![1, 3];
    let expected = 1;
    assert_eq!(expected, hamming_distance(input));
}