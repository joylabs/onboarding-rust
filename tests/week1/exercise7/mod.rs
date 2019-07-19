use onboarding_rust::week1::exercise7::title_to_number;

#[test]
fn test_week1_exercise7_example1() {
    let input = "A";
    let expected = 1;
    assert_eq!(expected, title_to_number(input.to_string()));
}

#[test]
fn test_week1_exercise7_example2() {
    let input = "AB";
    let expected = 28;
    assert_eq!(expected, title_to_number(input.to_string()));
}

#[test]
fn test_week1_exercise7_example3() {
    let input = "ZY";
    let expected = 701;
    assert_eq!(expected, title_to_number(input.to_string()));
}

#[test]
fn test_week1_exercise7_example4() {
    let input = "AAA";
    let expected = 703;
    assert_eq!(expected, title_to_number(input.to_string()));
}