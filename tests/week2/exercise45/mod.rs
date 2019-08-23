
#[test]
fn test_week3_exercise1_example1() {
    let input = vec![5,5,5,10,20];
    let output = true;
    assert_eq!(output, lemonade_change(input));
}

use onboarding_rust::week2::exercise45::lemonade_change;

#[test]
fn one_test_lemonade_change() {
    let input = vec![10,10];
    let output = false;
    assert_eq!(output, lemonade_change(input));
}

#[test]
fn two_test_lemonade_change() {
    let input = vec![5,5,10,10,20];
    let output = false;
    assert_eq!(output, lemonade_change(input));
}

#[test]
fn three_test_lemonade_change() {
    let input = vec![5,5,10];
    let output = true;
    assert_eq!(output, lemonade_change(input));
}

