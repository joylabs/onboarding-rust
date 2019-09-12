use onboarding_rust::week3::exercise13::rob;

#[test]
fn test_week3_exercise13_example1() {
    let input = vec![2,3,2];
    let expected = 3;
    assert_eq!(expected, rob(input));
}

#[test]
fn test_week3_exercise13_example2() {
    let input = vec![1,2,3,1];
    let expected = 4;
    assert_eq!(expected, rob(input));
}
