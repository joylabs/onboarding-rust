use onboarding_rust::week3::exercise12::min_distance;

#[test]
fn test_week3_exercise12_example1() {
    let word1 = "horse".to_owned();
    let word2 = "ros".to_owned();
    let expected = 3;
    assert_eq!(expected, min_distance(word1, word2));
}

#[test]
fn test_week3_exercise12_example2() {
    let word1 = "intention".to_owned();
    let word2 = "execution".to_owned();
    let expected = 5;
    assert_eq!(expected, min_distance(word1, word2));
}
