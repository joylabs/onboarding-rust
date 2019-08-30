pub fn my_pow(x: f64, n: i32) -> f64 {
    if n == 0 {
        1.0
    } else if n < 0 {
        trunc_to_five_decimal_precision(1.0 / elevate_number_to_n_power(x, x, n.abs()))
    } else {
        trunc_to_five_decimal_precision(elevate_number_to_n_power(x, x, n))
    }
}

fn elevate_number_to_n_power(acc: f64, x: f64, n: i32) -> f64 {
    if n == 1 {
        return acc;
    }
    elevate_number_to_n_power(acc * x, x, n - 1)
}

fn trunc_to_five_decimal_precision(x: f64) -> f64 {
    let base = (10.0 as f64).powi(5);
    f64::from((x * base) as i32) / base
}