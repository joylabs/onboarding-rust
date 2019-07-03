use onboarding_rust::week1::exercise1::array_by_parity;

#[test]
fn array_by_parity_test() {
    assert_eq!(vec![2, 4, 3, 1], array_by_parity(vec![3, 4, 2, 1]));
}
