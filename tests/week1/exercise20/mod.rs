use onboarding_rust::week1::exercise20::valid_anagram;

#[test]
fn test_week1_exercise20_example1() {
    let input_1 = "anagram".to_string();
    let input_2 = "nagaram".to_string();
    let expected = true;
    assert_eq!(expected, valid_anagram(input_1, input_2));
}

#[test]
fn test_week1_exercise20_example2() {
    let input_1 = "rat".to_string();
    let input_2 = "car".to_string();
    let expected = false;
    assert_eq!(expected, valid_anagram(input_1, input_2));
}

