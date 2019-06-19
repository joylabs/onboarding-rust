use onboarding_rust::week1::exercise19::exercise19;

#[test]
fn test_1_week1_exercise19() {
    let a = 19;
    assert_eq!(true, exercise19(a));
}

#[test]
fn test_2_week1_exercise19() {
    let a = 845;
    assert_eq!(false, exercise19(a));
}

#[test]
fn test_3_week1_exercise19() {
    let a = 860;
    assert_eq!(true, exercise19(a));
}