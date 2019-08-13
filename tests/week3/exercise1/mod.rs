use onboarding_rust::week3::exercise1::lemonade_change;

#[test]
fn test_week3_exercise1_example1() {
    let input = vec![5,5,5,10,20];
    let output = true;
    assert_eq!(output, lemonade_change(input));
}

#[test]
fn test_week3_exercise1_example2() {
    let input = vec![5,5,10];
    let output = true;
    assert_eq!(output, lemonade_change(input));
}

#[test]
fn test_week3_exercise1_example3() {
    let input = vec![10,10];
    let output = false;
    assert_eq!(output, lemonade_change(input));
}

#[test]
fn test_week3_exercise1_example4() {
    let input = vec![5,5,10,10,20];
    let output = false;
    assert_eq!(output, lemonade_change(input));
}
