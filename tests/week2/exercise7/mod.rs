use onboarding_rust::week2::exercise7::find_intersection;

#[test]
fn test_week2_exercise7_example1() {
    let nums1 = vec![1, 2, 2, 1];
    let nums2 = vec![2, 2];
    let expected = vec![2];
    assert_eq!(expected, find_intersection(nums1, nums2));
}

#[test]
fn test_week2_exercise7_example2() {
    let nums1 = vec![4, 9, 5];
    let nums2 = vec![9, 4, 9, 8, 4];
    let expected = vec![4, 9];
    assert_eq!(expected, find_intersection(nums1, nums2));
}