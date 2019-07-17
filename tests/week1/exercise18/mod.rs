use onboarding_rust::week1::exercise18::single_number;

#[test]
fn test_week1_single_number_array_length_3() {
    let input = vec![2, 2, 1];
    let output = 1;
    assert_eq!(output, single_number(input));
}

#[test]
fn test_week1_single_number_array_length_5() {
    let input = vec![4, 1, 2, 1, 2];
    let output = 4;
    assert_eq!(output, single_number(input));
}