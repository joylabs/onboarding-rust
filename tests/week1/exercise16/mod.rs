use onboarding_rust::week1::exercise16::hamming_distance;
#[test]
fn test_week1_exercise16_one_and_four() {
    let inputx = 1;
    let inputy = 4;
    let output = 2;
    assert_eq!(output, hamming_distance(inputx, inputy));
}

#[test]
fn test_week1_exercise16_ones() {
    let inputx = 7;
    let inputy = 0;
    let output = 3;
    assert_eq!(output, hamming_distance(inputx, inputy));
}

#[test]
fn test_week1_exercise16_sevens() {
    let inputx = 7;
    let inputy = 7;
    let output = 0;
    assert_eq!(output, hamming_distance(inputx, inputy));
}