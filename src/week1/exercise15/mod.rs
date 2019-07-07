const BITS: i32 = 32;
pub fn number_complement(n: i32) -> i32 {
    let number_binary_string = format!("{:b}", n);
    let zeros = BITS - number_binary_string.len() as i32;
    (!n) << zeros >> zeros
}