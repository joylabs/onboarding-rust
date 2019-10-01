use std::collections::HashSet;

pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let set = nums1
        .into_iter()
        .filter(|n| nums2.contains(n))
        .fold(&mut HashSet::new(), |acc, num| {
            acc.insert(num);
            acc
        })
        .clone();

    set.into_iter().collect()
}
