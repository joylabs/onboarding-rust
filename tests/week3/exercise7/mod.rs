use onboarding_rust::week3::exercise7::peak_index_in_mountain_array;

#[test]
fn test_week3_exercise7_example1() {
    let input = vec![0,1,0];
    let expected = 1;
    assert_eq!(expected, peak_index_in_mountain_array(input));
}

#[test]
fn test_week3_exercise7_example2() {
    let input = vec![0,2,1,0];
    let expected = 1;
    assert_eq!(expected, peak_index_in_mountain_array(input));
}
