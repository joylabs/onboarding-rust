use onboarding_rust::week2::exercise3::happy_numbers;

#[test]
fn test_week2_exercise3_example1() {

    let input = 19;
    let expected = true;

    assert_eq!(expected, happy_numbers(input));
}
#[test]
fn test_week2_exercise3_example2() {

    let input = 20;
    let expected = false;

    assert_eq!(expected, happy_numbers(input));
}
