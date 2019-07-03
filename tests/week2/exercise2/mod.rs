use onboarding_rust::week2::exercise2::count_unique_morse_words;

#[test]
fn test_week2_exercise2_example1() {
    let input = vec!["gin", "zen", "gig", "msg"];
    let expected = 2;
    assert_eq!(expected, count_unique_morse_words(input));
}

