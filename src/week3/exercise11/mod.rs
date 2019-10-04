pub fn max_profit(prices: Vec<i32>) -> i32 {
    if prices.is_empty() {
        return 0;
    }

    let min_price = prices[0];

    prices
        .into_iter()
        .skip(1)
        .fold((0, min_price), |(max_prof, min_price), price| {
            eval_price(max_prof, min_price, price)
        })
        .0
}

fn eval_price(max_prof: i32, min_price: i32, price: i32) -> (i32, i32) {
    if price < min_price {
        (max_prof, price)
    } else if price - min_price > max_prof {
        (price - min_price, min_price)
    } else {
        (max_prof, min_price)
    }
}
