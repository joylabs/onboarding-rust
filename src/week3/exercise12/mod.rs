pub fn max_profit(prices: Vec<i32>) -> i32 {
    if prices.is_empty() {
        return 0;
    }
    //[7, 1, 5, 3, 6, 4]
    let max = 0;
    let min = *prices.iter().max().unwrap();
    compare(max, min, &prices, 0)
}

pub fn compare(max: i32, min: i32, vec: &[i32], position: usize) -> i32 {
    if position >= vec.len() {
        return max;
    }
    if min > vec[position] {
        let min = vec[position];
        compare(max, min, vec, position + 1)
    } else if vec[position] - min > max {
        let max = vec[position] - min;
        compare(max, min, vec, position + 1)
    } else {
        compare(max, min, vec, position + 1)
    }
}
