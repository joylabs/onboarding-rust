use onboarding_rust::week2::exercise3::is_happy;

#[test]
fn test_week2_exercise3_nineteen() {
    let input = 19;
    let expected = true;
    assert_eq!(expected, is_happy(input));
}

#[test]
fn test_week2_exercise3_thirty() {
    let input = 30;
    let expected = false;
    assert_eq!(expected, is_happy(input));
}

#[test]
fn test_week2_exercise3_one() {
    let input = 1;
    let expected = true;
    assert_eq!(expected, is_happy(input));
}
