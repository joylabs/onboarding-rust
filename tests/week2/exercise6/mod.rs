use onboarding_rust::week2::exercise6::num_special_equiv_groups;

#[test]
fn test_week2_exercise6_3() {
    let input = vec![
        "a".to_string(),
        "b".to_string(),
        "c".to_string(),
        "a".to_string(),
        "c".to_string(),
        "c".to_string(),
    ];
    let expected = 3;
    assert_eq!(expected, num_special_equiv_groups(input));
}

#[test]
fn test_week2_exercise6_4() {
    let input = vec![
        "aa".to_string(),
        "bb".to_string(),
        "ab".to_string(),
        "ba".to_string(),
    ];
    let expected = 4;
    assert_eq!(expected, num_special_equiv_groups(input));
}

#[test]
fn test_week2_exercise6_3b() {
    let input = vec![
        "abc".to_string(),
        "acb".to_string(),
        "bac".to_string(),
        "bca".to_string(),
        "cab".to_string(),
        "cba".to_string(),
    ];
    let expected = 3;
    assert_eq!(expected, num_special_equiv_groups(input));
}

#[test]
fn test_week2_exercise6_1() {
    let input = vec![
        "abcd".to_string(),
        "cdab".to_string(),
        "adcb".to_string(),
        "cbad".to_string(),
    ];
    let expected = 1;
    assert_eq!(expected, num_special_equiv_groups(input));
}
