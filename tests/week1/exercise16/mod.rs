use onboarding_rust::week1::exercise16::hamming_distance;

#[test]
fn test_week1_hamming_distance() {
    let x = 1;
    let y = 4;
    let output = 2;
    assert_eq!(output, hamming_distance(x, y));
}