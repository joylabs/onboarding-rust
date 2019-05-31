use onboarding_rust::week1::exercise1::hello_world;

#[test]
fn test_week1_exercise1() {
    assert_eq!(vec![2, 4, 3, 1], hello_world(vec![3, 1, 2, 4]));
}