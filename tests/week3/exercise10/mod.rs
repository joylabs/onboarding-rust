use onboarding_rust::week3::exercise10::my_pow;

#[test]
fn test_week3_exercise10_example1() {
    let x = 2.00000;
    let n = 10;
    let expected: f64 = 1024.00000;
    assert_eq!(expected, my_pow(x, n));
}

#[test]
fn test_week3_exercise10_example2() {
    let x = 2.10000;
    let n = 3;
    let expected: f64 = 9.26100;
    assert_eq!(expected, my_pow(x, n));
}

#[test]
fn test_week3_exercise10_example3() {
    let x = 2.00000;
    let n = -2;
    let expected: f64 = 0.25000;
    assert_eq!(expected, my_pow(x, n));
}

#[test]
fn test_week3_exercise10_example4() {
    let x = 2.00000;
    let n = 0;
    let expected: f64 = 1.0;
    assert_eq!(expected, my_pow(x, n));
}