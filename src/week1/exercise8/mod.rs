pub fn power_of_two(input: i32) -> bool {
    let exponente;

    exponente = f64::from(input).log10() / 2f64.log10();

    if exponente.fract() == 0f64 {
        return true;
    }
    false
}
