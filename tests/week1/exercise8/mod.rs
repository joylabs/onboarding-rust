use onboarding_rust::week1::exercise8::is_power_of_two;

#[test]
fn test_week1_exercise8_1() {
    let input = 1;
    let output = true;
    assert_eq!(output, is_power_of_two(input));
}

#[test]
fn test_week1_exercise8_16() {
    let input = 16;
    let output = true;
    assert_eq!(output, is_power_of_two(input));
}

#[test]
fn test_week1_exercise8_218() {
    let input = 218;
    let output = false;
    assert_eq!(output, is_power_of_two(input));
}