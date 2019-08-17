use onboarding_rust::week1::exercise25::unique_morse;

#[test]
fn test_week1_exercise25_two() {
    let input = vec!["gin".to_string(), "zen".to_string(), "gig".to_string(), "msg".to_string()];
    let output = 2;
    assert_eq!(output, unique_morse(input));
}