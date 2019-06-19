use onboarding_rust::week1::exercise9::exercise9;

#[test]
fn test_1_week1_exercise9() {
    let a = 16; 
    assert_eq!(true, exercise9(a));
}

#[test]
fn test_2_week1_exercise9() {
    let a = 1; 
    assert_eq!(true, exercise9(a));
}

#[test]
fn test_3_week1_exercise9() {
    let a = 218; 
    assert_eq!(false, exercise9(a));
}