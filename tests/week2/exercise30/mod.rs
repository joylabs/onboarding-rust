use onboarding_rust::week2::exercise30::intersection_of_two;

#[test]
fn one_intersection_of_two() {
    let nums1 = vec![1, 2, 2, 1];
    let nums2 = vec![2, 2];
    let output = vec![2];
    assert!(contains_all(output, intersection_of_two(nums1, nums2) ));
}

#[test]
fn two_intersection_of_two() {
    let nums1 = vec![4, 9, 5];
    let nums2 = vec![9, 4, 9, 8, 4];
    let output = vec![4, 9];
    assert!(contains_all(output, intersection_of_two(nums1, nums2) ));
}

#[test]
fn three_intersection_of_two() {
    let nums1 = vec![4, 9, 5, 1, 8];
    let nums2 = vec![8, 6, 3, 4, 9];
    let output = vec![4, 8, 9];
    assert!(contains_all(output, intersection_of_two(nums1, nums2) ));
}

fn contains_all(a: Vec<i32>, b: Vec<i32>) -> bool {
    a.len() == b.len() && a.iter().all(|x| b.contains(x))
}