use onboarding_rust::week3::exercise15::is_match;

#[test]
fn test_is_match_true_1() {
    let s = "";
    let p = "a*b*c*";
    assert_eq!(true, is_match(s, p));
}

#[test]
fn test_is_match_true_two() {
    let s = "aab";
    let p = "c*a*b";
    assert_eq!(true, is_match(s, p));
}

#[test]
fn test_two_is_match_false() {
    let s = "mississippi";
    let p = "mis*is*p*.";
    assert_eq!(false, is_match(s, p));
}

#[test]
fn test_two_is_match_three() {
    let s = "mississippi";
    let p = "mis*is*ip*.";
    assert_eq!(true, is_match(s, p));
}

#[test]
fn test_is_matcha_true() {
    let s = "a";
    let p = "ab*a";
    assert_eq!(false, is_match(s, p));
}
