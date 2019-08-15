use onboarding_rust::week1::exercise7::title_to_number;

#[test]
fn test_week1_exercise7_a() {
    let input = String::from("A");;
    let output = 1;
    assert_eq!(output, title_to_number(input));
}

#[test]
fn test_week1_exercise7_ab() {
    let input = String::from("AB");;
    let output = 28;
    assert_eq!(output, title_to_number(input));
}

#[test]
fn test_week1_exercise7_zy() {
    let input = String::from("ZY");;
    let output = 701;
    assert_eq!(output, title_to_number(input));
}