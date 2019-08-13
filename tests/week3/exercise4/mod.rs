use onboarding_rust::week3::exercise4::find_min_arrow_shots;

#[test]
fn test_week3_exercise4_example1() {
    let input = vec![vec![10,16], vec![2,8], vec![1,6], vec![7,12]];
    let expected = 2;
    assert_eq!(expected, find_min_arrow_shots(input));
}
