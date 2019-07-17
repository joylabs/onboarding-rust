use std::cmp;

pub fn binary_gap(n: i32) -> i32 {
    let mut last = -1;
    let mut result = 0;
    for i in 0..32 {
        if ((n >> i) & 1) > 0 {
            if last >= 0 {
                result = cmp::max(result, i - last);
            }
            last = i;
        }
    }
    result
}