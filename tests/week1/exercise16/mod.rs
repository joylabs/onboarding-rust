use onboarding_rust::week1::exercise16::get_hamming_distance;

#[test]
fn test_week1_exercise16_example1() {
    let input = (1, 4);
    let expected = 2;
    assert_eq!(expected, get_hamming_distance(input));
}