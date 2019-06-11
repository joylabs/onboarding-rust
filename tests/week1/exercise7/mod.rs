use onboarding_rust::week1::exercise7::get_column_number;

#[test]
fn test_week1_exercise7_example1() {
    let input = "A";
    let expected = 1;
    assert_eq!(expected, get_column_number(input));
}

#[test]
fn test_week1_exercise7_example2() {
    let input = "AB";
    let expected = 28;
    assert_eq!(expected, get_column_number(input));
}

#[test]
fn test_week1_exercise7_example3() {
    let input = "ZY";
    let expected = 701;
    assert_eq!(expected, get_column_number(input));
}