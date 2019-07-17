use onboarding_rust::week1::exercise17::binary_gap;

#[test]
fn test_week1_binary_gap_of_two() {
    let input = 5;
    let output = 2;
    assert_eq!(output, binary_gap(input))
}

#[test]
fn test_week1_binary_gap_of_one() {
    let input = 6;
    let output = 1;
    assert_eq!(output, binary_gap(input))
}

#[test]
fn test_week1_binary_gap_of_zero() {
    let input = 8;
    let output = 0;
    assert_eq!(output, binary_gap(input))
}