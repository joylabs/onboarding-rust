pub fn single_number(nums: Vec<i32>) -> i32 {

    nums.iter().fold(0, |acc, n| acc ^ n)

}

pub fn single_number_2(nums: Vec<i32>) -> i32 {
    use std::collections::HashMap;
    let mut counter = HashMap::new();

    nums.iter().for_each(|x| {
        *counter.entry(x).or_insert(0) += 1;
    });
    **counter.iter().find(|(_, v)| **v == 1).unwrap().0

}

pub fn single_number_3(nums: Vec<i32>) -> i32 {
        let mut input = nums;
    input.sort();
    let mut unique = input.remove(0);
    let mut is_unique = true;
    for next in input {
        if unique == next {
            is_unique = false;
        } else {
            if is_unique {
                return unique;
            }
            is_unique = true;
        }
        unique = next;
    }
    unique
}
