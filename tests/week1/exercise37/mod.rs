use onboarding_rust::week1::exercise37::first_uniq_char;

#[test]
fn test_exercise37_first_unique_char_l() {
    let input = "leetcode".to_string();
    let output = 0;
    assert_eq!(output, first_uniq_char(input));
}

#[test]
fn test_exercise37_first_unique_char_v() {
    let input = "loveleetcode".to_string();
    let output = 2;
    assert_eq!(output, first_uniq_char(input));
}