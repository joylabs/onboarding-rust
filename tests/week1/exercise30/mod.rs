use onboarding_rust::week1::exercise30::intersection;

#[test]
fn test_intersection_one() {
    let nums1 = vec![1, 2, 2, 1];
    let nums2 = vec![2, 2];
    let output = vec![2];
    assert_eq!(output, intersection(nums1, nums2));
}

#[test]
fn test_intersection_two() {
    let nums1 = vec![4, 9, 5];
    let nums2 = vec![9, 4, 9, 8, 4];
    let output = vec![4, 9];
    assert_eq!(output, intersection(nums1, nums2));
}