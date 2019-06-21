use onboarding_rust::week1::exercise10::detect_capital;

#[test]
fn test_week1_exercise10_example1() {
    let input = "USA";
    let expected = true;
    assert_eq!(expected, detect_capital(input));
}

#[test]
fn test_week1_exercise10_example2() {
    let input = "FlaG";
    let expected = false;
    assert_eq!(expected, detect_capital(input));
}

#[test]
fn test_week1_exercise10_example3() {
    let input = "Flag";
    let expected = true;
    assert_eq!(expected, detect_capital(input));
}
#[test]
fn test_week1_exercise10_example4() {
    let input = "flag";
    let expected = true;
    assert_eq!(expected, detect_capital(input));
}