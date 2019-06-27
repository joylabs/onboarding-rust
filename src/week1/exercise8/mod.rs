pub fn power_of_two(input: i32) -> bool {
    if f64::from(input).log2().fract() > f64::from(0) {
        return false;
    }
    true
}