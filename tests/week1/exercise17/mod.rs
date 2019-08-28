use onboarding_rust::week1::exercise17::binary_gap;
#[test]
fn test_week1_exercise17_twenty_two() {
    let input = 22;
    let output = 2;
    assert_eq!(output, binary_gap(input));
}

#[test]
fn test_week1_exercise17_five() {
    let input = 5;
    let output = 2;
    assert_eq!(output, binary_gap(input));
}

#[test]
fn test_week1_exercise17_six() {
    let input = 6;
    let output = 1;
    assert_eq!(output, binary_gap(input));
}

#[test]
fn test_week1_exercise17_eight() {
    let input = 8;
    let output = 0;
    assert_eq!(output, binary_gap(input));
}

#[test]
fn test_week1_exercise17_three_four_three() {
    let input = 343;
    let output = 2;
    assert_eq!(output, binary_gap(input));
}