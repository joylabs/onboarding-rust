use onboarding_rust::week1::exercise29::special_equivalent;

#[test]
fn test_week1_special_equivalent_three_groups() {
    let input = vec![
        "abc".to_string(),
        "acb".to_string(),
        "bac".to_string(),
        "bca".to_string(),
        "cab".to_string(),
        "cba".to_string(),
    ];
    let output = 3;
    // Explanation: 3 groups ["abc","cba"], ["acb","bca"], ["bac","cab"]
    assert_eq!(output, special_equivalent(input));
}

#[test]
fn test_week1_special_equivalent_one_group() {
    let input = vec![
        "abcd".to_string(),
        "cdab".to_string(),
        "adcb".to_string(),
        "cbad".to_string(),
    ];
    let output = 1;
    // Explanation: 1 group ["abcd","cdab","adcb","cbad"]
    assert_eq!(output, special_equivalent(input));
}