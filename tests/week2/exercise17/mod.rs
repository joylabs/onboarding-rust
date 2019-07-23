use onboarding_rust::week2::exercise17::most_common_word;

#[test]
fn test_week2_exercise17_example1() {
    let paragraph = "Bob hit a ball, the hit BALL flew far after it was hit.".to_string();
    let banned = vec!["hit".to_string()];
    let output = "ball".to_string();
    assert_eq!(output, most_common_word(paragraph, banned));
}
