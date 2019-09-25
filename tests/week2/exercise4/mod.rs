use onboarding_rust::week2::exercise4::uncommon_from_sentences;

#[test]
fn test_week2_exercise4_ex1() {
    let input1 = "this apple is sweet".to_string();
    let input2 = "this apple is sour".to_string();
    let expected = vec!["sour".to_string(), "sweet".to_string()];
    assert_eq!(expected, uncommon_from_sentences(input1, input2));
}

#[test]
fn test_week2_exercise4_ex2() {
    let a = "apple apple".to_string();
    let b = "banana".to_string();
    let expected = vec!["banana".to_string()];
    assert_eq!(expected, uncommon_from_sentences(a, b));
}

#[test]
fn test_week2_exercise4_ex3() {
    let input1 = "this apple is sweet".to_string();
    let input2 = "this apple is sweet".to_string();
    let expected: Vec<String> = vec![];
    assert_eq!(expected, uncommon_from_sentences(input1, input2));
}
