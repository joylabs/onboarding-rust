pub fn my_pow(x: f64, n: i32) -> f64 {
    pow(x, n)
}

pub fn pow(x: f64, n: i32) -> f64 {
    if n == 0 {
        return 1.0;
    }
    let sum: f64 = pow(x, n / 2);
    if n % 2 == 0 {
        sum * sum
    } else if n > 0 {
        sum * sum * x
    } else {
        (sum * sum) / x
    }
}