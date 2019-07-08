use onboarding_rust::week2::exercise3::is_happy_number;

#[test]
fn test_week2_exercise3_example1() {
    let input = 19;
    let expected = true;
    assert_eq!(expected, is_happy_number(input));
}

