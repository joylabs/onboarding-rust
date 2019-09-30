use onboarding_rust::week3::exercise7::peak_index_in_mountain_array;

#[test]
fn test_peak_mountain_one() {
    let vec = vec![0, 1, 0];
    let output = 1;
    assert_eq!(output, peak_index_in_mountain_array(vec));
}

#[test]
fn test_peak_mountain_two() {
    let vec = vec![0, 2, 1, 0];
    let output = 1;
    assert_eq!(output, peak_index_in_mountain_array(vec));
}