use onboarding_rust::week1::exercise8::power_of_two;

#[test]
fn test_week1_exercise8_not_power_of_two() {
    let input = 218;
    let output = false;
    assert_eq!(output, power_of_two(input));
}

#[test]
fn test_week1_exercise8_power_of_two() {
    let input = 16;
    let output = true;
    assert_eq!(output, power_of_two(input));
}