use onboarding_rust::week1::exercise27::uncommon_words;

#[test]
fn test_week1_uncommon_words() {
    let input_a = String::from("this apple is sweet");
    let input_b = String::from("this apple is sour");
    let output = vec!["sweet".to_string(), "sour".to_string()];
    assert_eq!(output, uncommon_words(input_a, input_b));
}

#[test]
fn test_week1_uncommon_words_single_word() {
    let input_a = String::from("apple apple");
    let input_b = String::from("banana");
    let output = vec!["banana".to_string()];
    assert_eq!(output, uncommon_words(input_a, input_b));
}