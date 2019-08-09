use onboarding_rust::week1::exercise1::sort_array_by_parity;

#[test]
fn test_week1_exercise1() {
    let input = vec![3,1,2,4];
    let output = vec![2,4,3,1];
    assert_eq!(output, sort_array_by_parity(input));
}
