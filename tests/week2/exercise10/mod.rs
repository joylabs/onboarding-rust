use onboarding_rust::week2::exercise10::word_pattern;

#[test]
fn test_week2_exercise10_example1() {
    let pattern = String::from("abba");
    let my_str = String::from("dog cat cat dog");
    let expected = true;
    assert_eq!(expected, word_pattern(pattern, my_str));
}

#[test]
fn test_week2_exercise10_example2() {
    let pattern = String::from("abba");
    let my_str = String::from("dog cat cat fish");
    let expected = false;
    assert_eq!(expected, word_pattern(pattern, my_str));
}

#[test]
fn test_week2_exercise10_example3() {
    let pattern = String::from("aaaa");
    let my_str = String::from("dog cat cat dog");
    let expected = false;
    assert_eq!(expected, word_pattern(pattern, my_str));
}

#[test]
fn test_week2_exercise10_example4() {
    let pattern = String::from("abba");
    let my_str = String::from("dog dog dog dog");
    let expected = false;
    assert_eq!(expected, word_pattern(pattern, my_str));
}