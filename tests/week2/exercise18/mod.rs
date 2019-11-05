use onboarding_rust::week2::exercise18::frequency_sort;

#[test]
fn test_week2_exercise18_tree() {
    let input = "tree".to_string();
    let expected = "eert".to_string();
    assert!(contains_all(expected, frequency_sort(input)));
}

#[test]
fn test_week2_exercise18_cccaaa() {
    let input = "cccaaa".to_string();
    let expected = "cccaaa".to_string();
    assert!(contains_all(expected, frequency_sort(input)));
}

#[test]
fn test_week2_exercise18_Aabb() {
    let input = "Aabb".to_string();
    let expected = "bbAa".to_string();
    assert!(contains_all(expected, frequency_sort(input)));
}

fn contains_all(a: String, b: String) -> bool {
    a.len() == b.len() && a.chars().all(|x| b.contains(x))
}
