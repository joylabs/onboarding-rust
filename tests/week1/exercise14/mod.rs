use onboarding_rust::week1::exercise14::exercise14;

#[test]
fn test_1_week1_exercise14() {
    let a = vec![
        "flower".to_string(),
        "flow".to_string(),
        "flight".to_string(),
    ];
    let b = "fl".to_string();
    assert_eq!(b, exercise14(a));
}

#[test]
fn test_2_week1_exercise14() {
    let a = vec!["dog".to_string(), "racecar".to_string(), "car".to_string()];
    let b = "".to_string();
    assert_eq!(b, exercise14(a));
}