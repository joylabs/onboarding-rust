use onboarding_rust::week1::exercise10::detect_capital_use;

#[test]
fn test_week1_exercise10_usa() {
    let input = String::from("USA");
    let output = true;
    assert_eq!(output, detect_capital_use(input));
}

#[test]
fn test_week1_exercise10_flag() {
    let input = String::from("FlaG");
    let output = false;
    assert_eq!(output, detect_capital_use(input));
}

#[test]
fn test_week1_exercise10_google() {
    let input = String::from("Google");
    let output = true;
    assert_eq!(output, detect_capital_use(input));
}

#[test]
fn test_week1_exercise10_g() {
    let input = String::from("g");
    let output = true;
    assert_eq!(output, detect_capital_use(input));
}