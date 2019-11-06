pub fn my_pow(x: f64, n: i32) -> f64 {
    let is_neg = n < 0;
    let mut power = n;
    let mut base = x;
    let mut val = 1.0;

    while power != 0 {
        if power % 2 == 0 {
            power /= 2;
            base *= base;
        } else {
            if is_neg {
                power += 1;
            } else {
                power -= 1;  
            }
            
            val = val * base;

            power /= 2;
            base *= base;
        }
    }

    if is_neg {
        val = 1.0 / val;
    }

    trunc_to_five_decimal_precision(val)
}

fn trunc_to_five_decimal_precision(x: f64) -> f64 {
    let base = (10.0 as f64).powi(5);
    f64::from((x * base) as i32) / base
}