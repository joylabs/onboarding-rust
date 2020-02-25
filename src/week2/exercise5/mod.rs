pub fn distribute_candies(candies: Vec<i32>) -> i32 {
    let max = candies.len() / 2;
    let mut sort_candies = candies.clone();
    let different_candies;
    sort_candies.sort();
    sort_candies.dedup();
    different_candies = sort_candies.len();

    if max < different_candies{
         return max as i32;
    }

        different_candies as i32
}
