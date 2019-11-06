use onboarding_rust::week2::exercise17::most_common_word;

#[test]
fn test_week2_exercise17_ball() {
    let paragraph = "Bob hit a ball, the hit BALL flew far after it was hit.".to_string();
    let banned = vec!["hit".to_string()];
    let expected = "ball".to_string();
    assert_eq!(expected, most_common_word(paragraph, banned));
}

#[test]
fn test_week2_exercise17_bob() {
    let paragraph = "Bob hit a ball, the hit BALL flew far after it was hit by bob.".to_string();
    let banned = vec!["hit".to_string(), "ball".to_string()];
    let expected = "bob".to_string();
    assert_eq!(expected, most_common_word(paragraph, banned));
}

#[test]
fn test_week2_exercise17_ball2() {
    let paragraph = "Bob. hIt, baLl".to_string();
    let banned = vec!["bob".to_string(), "hit".to_string()];
    let expected = "ball".to_string();
    assert_eq!(expected, most_common_word(paragraph, banned));
}

#[test]
fn test_week2_exercise17_b() {
    let paragraph = "a, a, a, a, b,b,b,c, c".to_string();
    let banned = vec!["a".to_string()];
    let expected = "b".to_string();
    assert_eq!(expected, most_common_word(paragraph, banned));
}
