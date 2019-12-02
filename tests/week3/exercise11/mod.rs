use onboarding_rust::week3::exercise11::min_distance;

#[test]
fn test_min_distance_one() {
    let word_1 = "horse".to_string();
    let word_2 = "ros".to_string();
    assert_eq!(3, min_distance(word_1, word_2));
}

#[test]
fn test_min_distance_two() {
    let word_1 = "intention".to_string();
    let word_2 = "execution".to_string();
    assert_eq!(5, min_distance(word_1, word_2));
}

#[test]
fn test_min_distance_onw() {
    let word_1 = "a".to_string();
    let word_2 = "ab".to_string();
    assert_eq!(1, min_distance(word_1, word_2));
}

#[test]
fn four_11() {
    let word_1 = "mart".to_string();
    let word_2 = "karma".to_string();
    assert_eq!(3, min_distance(word_1, word_2));
}
