pub fn exercise15(x: i32) -> i32 {
    let z = format!("{:b}", x);
    let mut sum: i32 = 0;
    let binary: i32 = 2;
    for (j, i) in z.chars().rev().enumerate() {
        if '0' == i {
            sum += binary.pow(j as u32);
        }
    }
    sum
}