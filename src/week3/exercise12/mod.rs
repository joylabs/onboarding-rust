pub fn max_profit(prices: Vec<i32>) -> i32 {
    if prices.is_empty() {
        return 0;
    }
    //[7, 1, 5, 3, 6, 4]
    let mut max = 0;
    let mut min = *prices.iter().max().unwrap();

    for v in prices {
        if min > v {
            min = v;
        } else if v - min > max {
            max = v - min;
        }
    }    max
}
