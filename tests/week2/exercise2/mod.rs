use onboarding_rust::week2::exercise2::unique_morse_representations;

#[test]
fn test_week2_exercise1_example1() {
    let input = vec![
        "gin".to_string(),
        "zen".to_string(),
        "gig".to_string(),
        "msg".to_string(),
    ];

    let expected = 2;

    assert_eq!(expected, unique_morse_representations(input));
}
