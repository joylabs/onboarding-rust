use onboarding_rust::week1::exercise16::hamming_distance;

#[test]
fn test_week1_exercise16_example1() {

    let input = vec![1, 4];
    let expected = 2;
    assert_eq!(expected, hamming_distance(input));
}
#[test]
fn test_week1_exercise16_example2() {

    let input = vec![1, 3];
    let expected = 1;
    assert_eq!(expected, hamming_distance(input));
}