use onboarding_rust::week3::exercise2::is_subsequence;

#[test]
fn test_is_subsequence_one() {
    let s = "abc".to_string();
    let t = "".to_string();
    assert!(!is_subsequence(s, t));
}

#[test]
fn test_is_subsequence_two() {
    let s = "axc".to_string();
    let t = "ahbgdc".to_string();
    assert!(!is_subsequence(s, t));
}