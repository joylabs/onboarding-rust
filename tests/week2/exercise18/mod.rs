use onboarding_rust::week2::exercise18::frequency_sort;

#[test]
fn test_week2_exercise18_example1() {
    let input = "tree".to_string();
    let expected = vec!["eert".to_string(), "eetr".to_string()];
    let output = frequency_sort(input);
    assert!(expected.contains(&output));
}

#[test]
fn test_week2_exercise18_example2() {
    let input = "cccaaa".to_string();
    let expected = vec!["cccaaa".to_string(), "aaaccc".to_string()];
    let output = frequency_sort(input);
    assert!(expected.contains(&output));
}

#[test]
fn test_week2_exercise18_example3() {
    let input = "Aabb".to_string();
    let expected = vec!["bbAa".to_string(), "bbaA".to_string()];
    let output = frequency_sort(input);
    assert!(expected.contains(&output));
}
