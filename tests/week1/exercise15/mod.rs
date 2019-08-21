use onboarding_rust::week1::exercise15::find_complement;
#[test]
fn test_week1_exercise15_example1() {
    let input = 5;
    let output = 2;
    assert_eq!(output, find_complement(input));
}