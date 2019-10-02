use std::collections::HashSet;

pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    nums1
        .into_iter()
        .filter(|n| nums2.contains(n))
        .collect::<HashSet<i32>>()
        .into_iter()
        .collect::<Vec<i32>>()
}
