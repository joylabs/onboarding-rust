use onboarding_rust::week2::exercise32::word_pattern;


#[test]
fn one_word_pattern() {
    let patter = "abba".to_string();
    let string_str = "dog cat cat dog".to_string();
    assert!(word_pattern(patter, string_str));
}

#[test]
fn two_word_pattern() {
    let patter = "abba".to_string();
    let string_str = "dog cat cat fish".to_string();
    assert!(!word_pattern(patter, string_str));
}

#[test]
fn three_word_pattern() {
    let patter = "aaaa".to_string();
    let string_str = "dog cat cat dog".to_string();
    assert!(!word_pattern(patter, string_str));
}

#[test]
fn four_word_pattern() {
    let patter = "abba".to_string();
    let string_str = "dog dog dog dog".to_string();
    assert!(!word_pattern(patter, string_str));
}