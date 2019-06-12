use onboarding_rust::week1::exercise16::exercise16;

#[test]
fn test_1_week1_exercise16() {
    let a: i32 = 4;
    let b: i32 = 1;
    let c: i32 = 2;
    assert_eq!(c, exercise16(a, b));
}

#[test]
fn test_2_week1_exercise16() {
    let a: i32 = 3;
    let b: i32 = 14;
    let c: i32 = 3;
    assert_eq!(c, exercise16(a, b));
}