use std::collections::HashMap;

pub fn my_pow(x: f64, n: i32) -> f64 {
    let mut pow_mem: HashMap<i32, f64> = HashMap::new();

    if n < 0 {
        trunc_to_five_decimal_precision(1.0 / pow_to_n(x, n.abs(), &mut pow_mem))
    } else {
        trunc_to_five_decimal_precision(pow_to_n(x, n, &mut pow_mem))
    }
}

fn pow_to_n(base: f64, n: i32, pow_mem: &mut HashMap<i32, f64>) -> f64 {
    if n == 0 {
        1.0
    } else if pow_mem.contains_key(&n) {
        *pow_mem.get(&n).unwrap()
    } else if (n % 2) == 0 {
        let val = pow_to_n(base, n / 2, pow_mem);
        *pow_mem.entry(n).or_insert(val * val)
    } else {
        let val = pow_to_n(base, n / 2, pow_mem);
        *pow_mem.entry(n).or_insert(val * val * base)
    }
}

fn trunc_to_five_decimal_precision(x: f64) -> f64 {
    let base = (10.0 as f64).powi(5);
    f64::from((x * base) as i32) / base
}