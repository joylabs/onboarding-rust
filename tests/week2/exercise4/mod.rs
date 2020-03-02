use onboarding_rust::week2::exercise4::uncommon_sentences_words;

#[test]
fn test_week2_exercise4_example1() {
    let input_a = "this apple is sweet".to_string();
    let input_b = "this apple is sour".to_string();

    let expected = vec!["sour".to_string(), "sweet".to_string()];

    assert_eq!(expected, uncommon_sentences_words(input_a, input_b));
}

#[test]
fn test_week2_exercise4_example2() {
    let input_a = "apple apple".to_string();
    let input_b = "banana".to_string();

    let expected = vec!["banana".to_string()];

    assert_eq!(expected, uncommon_sentences_words(input_a, input_b));
}
