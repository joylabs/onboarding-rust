use std::collections::HashMap;

pub fn single_number(nums: Vec<i32>) -> i32 {
    let mut input = nums;

    input.sort();

    let mut counter = HashMap::new();


    input.iter().for_each(|x| {
        *counter.entry(x).or_insert(0) += 1;
    });
    print!("{:?}", counter);
    **counter.iter().find(|(_, v)| **v == 1).unwrap().0

}