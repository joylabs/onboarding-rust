pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let mut result = nums2
        .into_iter()
        .filter(|n| nums1.contains(n))
        .collect::<Vec<i32>>();
    result.sort();
    result.dedup();
    result
}