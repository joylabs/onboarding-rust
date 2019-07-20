use onboarding_rust::week1::exercise20::valid_anagram;

#[test]
fn week1_test_valid_anagram() {
    let s = String::from("anagram");
    let t = String::from("nagaram");
    let output = true;
    assert_eq!(output, valid_anagram(s, t));
}

#[test]
fn week1_test_not_valid_anagram() {
    let s = String::from("rat");
    let t = String::from("car");
    let output = false;
    assert_eq!(output, valid_anagram(s, t));
}

#[test]
fn week1_test_valid_anagram_different_length() {
    let s = String::from("Hello");
    let t = String::from("Helloo");
    let output = false;
    assert_eq!(output, valid_anagram(s, t));
}