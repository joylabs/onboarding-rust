use onboarding_rust::week1::exercise39::find_the_difference;

#[test]
fn test_find_the_difference_letter_e() {
    let s = "abcd".to_string();
    let t = "abcde".to_string();
    let output = 'e';
    assert_eq!(output, find_the_difference(s, t));
}

#[test]
fn test_find_the_difference_letter_z() {
    let s = "badc".to_string();
    let t = "abzcd".to_string();
    let output = 'z';
    assert_eq!(output, find_the_difference(s, t));
}

#[test]
fn test_find_the_difference_letter_b() {
    let s = "abcd".to_string();
    let t = "abbcde".to_string();
    let output = 'b';
    assert_eq!(output, find_the_difference(s, t));
}

#[test]
fn test_find_the_difference_letter_d() {
    let s = "azbecy".to_string();
    let t = "azbdecy".to_string();
    let output = 'd';
    assert_eq!(output, find_the_difference(s, t));
}

#[test]
fn test_find_the_difference_letter_r() {
    let s = "azbecy".to_string();
    let t = "zabryce".to_string();
    let output = 'r';
    assert_eq!(output, find_the_difference(s, t));
}