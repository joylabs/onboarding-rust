use onboarding_rust::week1::exercise12::exercise12;

#[test]
fn test_1_week1_exercise12() {
    let a = String::from("A man, a plan, a canal: Panama");
    assert_eq!(true, exercise12(a));
}

#[test]
fn test_2_week1_exercise12() {
    let a = String::from("race a car");
    assert_eq!(false, exercise12(a));
}
