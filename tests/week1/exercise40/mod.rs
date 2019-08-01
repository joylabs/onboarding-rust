use onboarding_rust::week1::exercise40::most_common_word;

#[test]
fn test_week1_exercise40_most_common_word_ball() {
    let paragraph = "Bob hit a ball, the hit BALL flew far after it was hit.".to_string();
    let banned = vec!["hit".to_string()];
    let output = "ball".to_string();
    assert_eq!(output, most_common_word(paragraph, banned));
}

#[test]
fn test_week1_exercise40_most_common_word_b() {
    let paragraph = "a, a, a, a, b,b,b,c, c".to_string();
    let banned = vec!["a".to_string()];
    let output = "b".to_string();
    assert_eq!(output, most_common_word(paragraph, banned));
}