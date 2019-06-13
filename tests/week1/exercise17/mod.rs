use onboarding_rust::week1::exercise17::exercise17;

#[test]
fn test_1_week1_exercise17() {
    let a = 5;
    let b = 2;
    assert_eq!(b, exercise17(a));
}


#[test]
fn test_2_week1_exercise17() {
    let a = 6;
    let b = 1;
    assert_eq!(b, exercise17(a));
}


#[test]
fn test_3_week1_exercise17() {
    let a = 22;
    let b = 2;
    assert_eq!(b, exercise17(a));
}