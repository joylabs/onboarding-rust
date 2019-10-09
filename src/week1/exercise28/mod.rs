use std::collections::HashSet;

pub fn distribute_candies(candies: Vec<i32>) -> i32 {
    let number_candies = candies.len() / 2;
    let kinds: HashSet<i32> = candies.into_iter().collect();
    if number_candies <= kinds.len() {
        return number_candies as i32;
    }
    kinds.len() as i32
}