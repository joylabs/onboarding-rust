use onboarding_rust::week3::exercise16::rob;

#[test]
fn test_rob_one() {
    let input = vec![2, 3, 2];
    let expect = 3;
    assert_eq!(expect, rob(input));
}

#[test]
fn test_rob_two() {
    let input = vec![1, 2, 3, 1];
    let expect = 4;
    assert_eq!(expect, rob(input));
}

#[test]
fn test_rob_three() {
    let input = vec![1, 2, 3, 1, 4, 5, 6, 1, 3, 6, 9];
    let expect = 25;
    assert_eq!(expect, rob(input));
}

#[test]
fn test_rob_four() {
    let input = vec![6, 3, 10, 8, 2, 10, 3, 5, 10, 5, 3];
    let expect = 36;
    assert_eq!(expect, rob(input));
}

#[test]
fn test_rob_five() {
    let input = vec![1, 1, 9, 1, 2, 5];
    let expect = 14;
    assert_eq!(expect, rob(input));
}
