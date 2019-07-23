use onboarding_rust::week2::exercise16::find_the_difference;

#[test]
fn test_week2_exercise16_example1() {
    let s = "abcd".to_string();
    let t = "abcde".to_string();
    let output = 'e';
    assert_eq!(output, find_the_difference(s, t));
}

#[test]
fn test_week2_exercise16_example2() {
    let s = String::new();
    let t = "y".to_string();
    let output = 'y';
    assert_eq!(output, find_the_difference(s, t));
}

#[test]
fn test_week2_exercise16_example3() {
    let s = "a".to_string();
    let t = "aa".to_string();
    let output = 'a';
    assert_eq!(output, find_the_difference(s, t));
}
