use onboarding_rust::week1::exercise15::exercise15;

#[test]
fn test_1_week1_exercise15() {
    let a = 5;
    let b = 2;
    assert_eq!(b, exercise15(a));
}

#[test]
fn test_2_week1_exercise15() {
    let a = 1;
    let b = 0;
    assert_eq!(b, exercise15(a));
}

#[test]
fn test_3_week1_exercise15() {
    let a = 8;
    let b = 7;
    assert_eq!(b, exercise15(a));
}
