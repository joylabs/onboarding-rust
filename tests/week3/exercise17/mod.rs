use onboarding_rust::week3::exercise17::min_distance;

#[test]
fn test_week3_exercise17_example1() {
    let word1 = "horse".to_owned();
    let word2 = "ros".to_owned();
    let expected = 3;
    assert_eq!(expected, min_distance(word1, word2));
}

#[test]
fn test_week3_exercise17_example2() {
    let word1 = "intention".to_owned();
    let word2 = "execution".to_owned();
    let expected = 5;
    assert_eq!(expected, min_distance(word1, word2));
}

#[test]
fn test_week3_exercise17_example3() {
    let word1 = "".to_owned();
    let word2 = "".to_owned();
    let expected = 0;
    assert_eq!(expected, min_distance(word1, word2));
}

#[test]
fn test_week3_exercise17_example4() {
    let word1 = "".to_owned();
    let word2 = "a".to_owned();
    let expected = 1;
    assert_eq!(expected, min_distance(word1, word2));
}

#[test]
fn test_week3_exercise17_example5() {
    let word1 = "a".to_owned();
    let word2 = "a".to_owned();
    let expected = 0;
    assert_eq!(expected, min_distance(word1, word2));
}

#[test]
fn test_week3_exercise17_example6() {
    let word1 = "a".to_owned();
    let word2 = "ab".to_owned();
    let expected = 1;
    assert_eq!(expected, min_distance(word1, word2));
}

#[test]
fn test_week3_exercise17_example7() {
    let word1 = "ab".to_owned();
    let word2 = "bc".to_owned();
    let expected = 2;
    assert_eq!(expected, min_distance(word1, word2));
}
