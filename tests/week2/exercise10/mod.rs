use onboarding_rust::week2::exercise10::word_pattern;

#[test]
fn test_week2_exercise10_exercise1() {
    let pattern = "abba".to_string();
    let string = "dog cat cat dog".to_string();
    let expected = true;
    assert_eq!(expected, word_pattern(pattern, string));
}

#[test]
fn test_week2_exercise10_exercise2() {
    let pattern = "abba".to_string();
    let string = "dog cat cat fish".to_string();
    let expected = false;
    assert_eq!(expected, word_pattern(pattern, string));
}

#[test]
fn test_week2_exercise10_exercise3() {
    let pattern = "aaaa".to_string();
    let string = "dog cat cat dog".to_string();
    let expected = false;
    assert_eq!(expected, word_pattern(pattern, string));
}

#[test]
fn test_week2_exercise10_exercise4() {
    let pattern = "abba".to_string();
    let string = "dog dog dog dog".to_string();
    let expected = false;
    assert_eq!(expected, word_pattern(pattern, string));
}

#[test]
fn test_week2_exercise10_exercise5() {
    let pattern = "aaa".to_string();
    let string = "aa aa aa aa".to_string();
    let expected = false;
    assert_eq!(expected, word_pattern(pattern, string));
}
