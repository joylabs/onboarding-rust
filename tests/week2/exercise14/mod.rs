use onboarding_rust::week2::exercise14::first_unique_char;

#[test]
fn test_week2_exercise14_example1() {
    let input  = "leetcode";
    let output = 0;
    assert_eq!(output, first_unique_char(input));
}

#[test]
fn test_week2_exercise14_example2() {
    let input  = "loveleetcode";
    let output = 2;
    assert_eq!(output, first_unique_char(input));
}
