
use std::cmp;
use std::collections::HashSet;

pub fn distribute_candies(candies: Vec<i32>) -> i32 {
    
    let distance = candies.len();
    let set: HashSet<i32> = candies.into_iter().map(|kind| kind).collect();
    cmp::min((distance/2), set.len()) as i32

}