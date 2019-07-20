use onboarding_rust::week1::exercise10::detect_capital_use;

#[test]
fn test_week1_exercise10_all_caps() {
    let input: String = String::from("USA");
    let output = true;
    assert_eq!(output, detect_capital_use(input));
}

#[test]
fn test_week1_exercise10_all_lower_case() {
    let input: String = String::from("hello");
    let output = true;
    assert_eq!(output, detect_capital_use(input));
}

#[test]
fn test_week1_exercise10_first_letter_capital() {
    let input: String = String::from("Joylabs");
    let output = true;
    assert_eq!(output, detect_capital_use(input));
}

#[test]
fn test_week1_exercise10_two_capitals() {
    let input: String = String::from("FaiL");
    let output = false;
    assert_eq!(output, detect_capital_use(input));
}