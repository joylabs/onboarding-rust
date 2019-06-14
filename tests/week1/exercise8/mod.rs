use onboarding_rust::week1::exercise8::is_power_of_two;

#[test]
fn test_week1_exercise8_example1() {

    let input = 1;
    let expected = true;
    assert_eq!(expected, is_power_of_two(input));
}

#[test]
fn test_week1_exercise8_example2() {

    let input = 16;
    let expected = true;
    assert_eq!(expected, is_power_of_two(input));
}

#[test]
fn test_week1_exercise8_example3() {

    let input = 218;
    let expected = false;
    assert_eq!(expected, is_power_of_two(input));
}