use onboarding_rust::week3::exercise8::array_pair_sum;

#[test]
fn test_week3_exercise8_example1() {
    let input = vec![1,4,3,2];
    let expected = 4;
    assert_eq!(expected, array_pair_sum(input));
}
