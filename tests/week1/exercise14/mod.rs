use onboarding_rust::week1::exercise14::common_prefix;

#[test]
fn test_week1_exercise14_common_prefix() {
    let input = vec![
        "flower".to_string(),
        "flow".to_string(),
        "flight".to_string(),
    ];
    let output = String::from("fl");
    assert_eq!(output, common_prefix(input));
}

#[test]
fn test_week1_exercise14_no_common() {
    let input = vec!["dog".to_string(), "racecar".to_string(), "car".to_string()];
    let output = String::from("");
    assert_eq!(output, common_prefix(input));
}