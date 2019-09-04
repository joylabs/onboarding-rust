use onboarding_rust::week2::exercise14::first_unique_char;

#[test]
fn test_week2_exercise14_example1() {
    let input = "leetcode".to_string();
    let output = 0;
    assert_eq!(output, first_unique_char(input));
}

#[test]
fn test_week2_exercise14_example2() {
    let input = "loveleetcode".to_string();
    let output = 2;
    assert_eq!(output, first_unique_char(input));
}

#[test]
fn test_week2_exercise14_example3() {
    let input = "cc".to_string();
    let output = -1;
    assert_eq!(output, first_unique_char(input));
}

#[test]
fn test_week2_exercise14_example4() {
    let input = "aadadaad".to_string();
    let output = -1;
    assert_eq!(output, first_unique_char(input));
}
