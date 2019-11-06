pub fn my_pow(x: f64, n: i32) -> f64 {
    let is_odd = n.abs() % 2 != 0;
    let is_neg = n < 0;
    let mut val = pow_to_n(x * x, (n / 2).abs());

    if is_odd {
        val *= x;
    }

    if is_neg {
        val = 1.0 / val;
    }

    trunc_to_five_decimal_precision(val)
}

fn pow_to_n(base: f64, n: i32) -> f64 {
    if n == 0 {
        1.0
    } else if (n % 2) == 0 {
        let val = pow_to_n(base, n / 2);
        val * val
    } else {
        let val = pow_to_n(base, n / 2);
        val * val * base
    } 
}

fn trunc_to_five_decimal_precision(x: f64) -> f64 {
    let base = (10.0 as f64).powi(5);
    f64::from((x * base) as i32) / base
}