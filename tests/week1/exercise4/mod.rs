use onboarding_rust::week1::exercise4::exercise4;

#[test]
fn test_week1_exercise1() {
    let a:i32 = 1;
    let b:i32 = 22;
    let v = vec! [1, 2, 3, 4, 5, 6, 7, 8, 9, 11, 12, 15, 22];
    assert_eq!(v, exercise4(a,b));
}