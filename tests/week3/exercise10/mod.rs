use onboarding_rust::week3::exercise10::my_pow;

#[test]
fn test_powx_1() {
    let x = 2.00000;
    let n = 10;
    let output: f64 = 1024.00000;
    assert_eq!(convert_to_i32(output), convert_to_i32(my_pow(x, n)));
}

#[test]
fn test_powx_2() {
    let x = 2.10000;
    let n = 3;
    let output: f64 = 9.26100;
    assert_eq!(convert_to_i32(output), convert_to_i32(my_pow(x, n)));
}

#[test]
fn test_powx_3() {
    let x = 2.00000;
    let n = -2;
    let output: f64 = 0.25000;
    assert_eq!(convert_to_i32(output), convert_to_i32(my_pow(x, n)));
}

//Using 5 decimal precision
fn convert_to_i32(x: f64) -> i32 {
    x as i32
}