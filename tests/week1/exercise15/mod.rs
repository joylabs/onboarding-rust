use onboarding_rust::week1::exercise15::number_complement;

#[test]
fn test_week1_number_complement_3_bits() {
    let input = 5;
    let output = 2;
    assert_eq!(output, number_complement(input));
}

#[test]
fn test_week1_number_complement_1_bits() {
    let input = 1;
    let output = 0;
    assert_eq!(output, number_complement(input));
}