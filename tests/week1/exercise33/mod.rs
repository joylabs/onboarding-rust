use onboarding_rust::week1::exercise33::word_pattern;

#[test]
fn test_e33_word_pattern_2letters_match() {
    let pattern = "abba".to_string(); 
    let _str = "dog cat cat dog".to_string();
    let output = true;
    assert_eq!(output, word_pattern(pattern, _str));
}

#[test]
fn test_e33_word_pattern_2letters_mismatch() {
    let pattern = "abba".to_string();
    let _str = "dog cat cat fish".to_string();
    let output = false;
    assert_eq!(output, word_pattern(pattern, _str));
}

#[test]
fn test_e33_word_pattern_1letter_4times_mismatch() {
    let pattern = "aaaa".to_string();
    let _str = "dog cat cat dog".to_string();
    let output = false;
    assert_eq!(output, word_pattern(pattern, _str));
}

#[test]
fn test_e33_word_pattern_2letters_mismtach() {
    let pattern = "abba".to_string();
    let _str = "dog dog dog dog".to_string();
    let output = false;
    assert_eq!(output, word_pattern(pattern, _str));
}