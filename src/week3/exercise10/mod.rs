pub fn my_pow(x: f64, n: i32) -> f64 {
    if n == 0 {
        1.0
    } else if n < 0 {
        trunc_to_five_decimal_precision(1.0 / pow_to_n(x, x, n.abs()))
    } else {
        trunc_to_five_decimal_precision(pow_to_n(x, x, n))
    }
}

fn pow_to_n(acc: f64, base: f64, n: i32) -> f64 {
    if n == 1 {
        return acc;
    }
    pow_to_n(acc * base, base, n - 1)
}

fn trunc_to_five_decimal_precision(x: f64) -> f64 {
    let base = (10.0 as f64).powi(5);
    f64::from((x * base) as i32) / base
}