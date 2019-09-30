use std::cmp;
use std::collections::HashSet;
use std::iter::FromIterator;

pub fn distribute_candies(candies: Vec<i32>) -> i32 {
    let kinds: HashSet<i32> = HashSet::from_iter(candies.iter().cloned());
    cmp::min((candies.len() / 2) as i32, kinds.len() as i32)
}
