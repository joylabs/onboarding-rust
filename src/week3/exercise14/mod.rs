pub fn my_pow(mut x: f64, mut n: i64) -> f64 {
    let mut result: f64 = 1.0;
    let is_negative = n > 0;

    if n == 0 {
        return 1.0;
    } else {
        if !is_negative {
            n *= -1;
        }
        while n > 0 {
            if n % 2 != 0 {
                result *= x;
            }
            x = x * x;
            n /= 2;
        }
        if !is_negative {
            result = 1_f64 / result;
        }
    }
    result
}
