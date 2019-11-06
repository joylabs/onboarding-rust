use onboarding_rust::week3::exercise15::my_pow;

#[test]
fn test_week3_exercise15_example1() {
    let x = 2.00000;
    let n = 10;
    let expected: f64 = 1024.00000;
    let result = my_pow(x, n);
    assert_eq!(convert_to_i32(expected), convert_to_i32(result));
}

#[test]
fn test_week3_exercise15_example2() {
    let x = 2.10000;
    let n = 3;
    let expected: f64 = 9.26100;
    let result = my_pow(x, n);
    assert_eq!(convert_to_i32(expected), convert_to_i32(result));
}

#[test]
fn test_week3_exercise15_example3() {
    let x = 2.00000;
    let n = -2;
    let expected: f64 = 0.25000;
    let result = my_pow(x, n);
    assert_eq!(convert_to_i32(expected), convert_to_i32(result));
}

#[test]
fn test_week3_exercise15_example4() {
    let x = 2.00000;
    let n = 0;
    let expected: f64 = 1.0;
    let result = my_pow(x, n);
    assert_eq!(convert_to_i32(expected), convert_to_i32(result));
}

#[test]
fn test_week3_exercise15_example5() {
    let x = 2.00000;
    let n = 1;
    let expected: f64 = 2.0;
    let result = my_pow(x, n);
    assert_eq!(convert_to_i32(expected), convert_to_i32(result));
}

#[test]
fn test_week3_exercise15_example6() {
    let x = 2.00000;
    let n = 3;
    let expected: f64 = 8.0;
    let result = my_pow(x, n);
    assert_eq!(convert_to_i32(expected), convert_to_i32(result));
}

#[test]
fn test_week3_exercise15_example7() {
    let x = 8.66731;
    let n = 4;
    let expected: f64 = 5643.35434;
    let result = my_pow(x, n);
    assert_eq!(convert_to_i32(expected), convert_to_i32(result));
}

#[test]
fn test_week3_exercise15_example8() {
    let x = 8.95371;
    let n = -1;
    let expected: f64 = 0.11168;
    let result = my_pow(x, n);
    assert_eq!(convert_to_i32(expected), convert_to_i32(result));
}

//Using 5 decimal precision
fn convert_to_i32(x: f64) -> i32 {
    let base = (10.0 as f64).powi(5);
    (x * base) as i32
}