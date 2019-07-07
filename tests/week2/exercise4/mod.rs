use onboarding_rust::week2::exercise4::get_uncommon_words;

#[test]
fn test_week2_exercise4_example1() {
    let a = "this apple is sweet".to_string();
    let b = "this apple is sour".to_string();
    let expected = vec!["sweet".to_string(), "sour".to_string()];
    assert_eq!(expected, get_uncommon_words(a, b));
}

#[test]
fn test_week2_exercise4_example2() {
    let a = "apple apple".to_string();
    let b = "banana".to_string();
    let expected = vec!["banana".to_string()];
    assert_eq!(expected, get_uncommon_words(a, b));
}
