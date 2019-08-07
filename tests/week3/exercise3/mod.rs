use onboarding_rust::week3::exercise3::is_subsequence;

#[test]
fn test_week3_exercise3_example1() {
    let s = "abc".to_string();
    let t = "ahbgdc".to_string();
    let expected = true;
    assert_eq!(expected, is_subsequence(s, t));
}

#[test]
fn test_week3_exercise3_example2() {
    let s = "axc".to_string();
    let t = "ahbgdc".to_string();
    let expected = false;
    assert_eq!(expected, is_subsequence(s, t));
}
