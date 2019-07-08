use onboarding_rust::week1::exercise26::is_happy_number;

#[test]
fn test_week1_happy_number_true() {
    let input = 19;
    let output = true;
    assert_eq!(output, is_happy_number(input));
}