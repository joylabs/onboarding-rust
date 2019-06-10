use onboarding_rust::week1::exercise8::exercise8;

#[test]
fn test_1_week1_exercise8() {
    let a = "A".to_string();
    let b = 1;
    assert_eq!(b, exercise8(a));
}

#[test]
fn test_2_week1_exercise8() {
    let a = "AB".to_string();
    let b = 28;
    assert_eq!(b, exercise8(a));
}

#[test]
fn test_3_week1_exercise8() {
    let a = "ZY".to_string();
    let b = 701;
    assert_eq!(b, exercise8(a));
}