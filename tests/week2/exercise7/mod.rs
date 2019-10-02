use onboarding_rust::week2::exercise7::intersection;

#[test]
fn test_week2_exercise7_2() {
    let input1 = vec![1, 2, 2, 1];
    let input2 = vec![2, 2];
    let expected = vec![2];
    let mut ordered_output = intersection(input1, input2);
    ordered_output.sort();
    assert_eq!(expected, ordered_output);
}

#[test]
fn test_week2_exercise7_94() {
    let input1 = vec![4, 9, 5];
    let input2 = vec![9, 4, 9, 8, 4];
    let expected = vec![4, 9];
    let mut ordered_output = intersection(input1, input2);
    ordered_output.sort();
    assert_eq!(expected, ordered_output);
}
