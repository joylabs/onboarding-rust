use onboarding_rust::week1::exercise22::exercise22;

#[test]
fn test_1_week1_exercise22() {
    let a = vec![vec![1,1,0],vec![1,1,0],vec![0,0,1]];
    let b = 2;
    assert_eq!(b, exercise22(a));
}
