use onboarding_rust::week2::exercise41::frequency_sort;

#[test]
fn one_frecuency_sort() {
    let input = "tree".to_string();
    let output = "eert".to_string();
    assert!(contains_all(&output, &frequency_sort(input)));
}

#[test]
fn two_frecuency_sort() {
    let input = "cccaaa".to_string();
    let output = "cccaaa".to_string();
    assert!(contains_all(&output, &frequency_sort(input)));
}

#[test]
fn three_frecuency_sort() {
    let input = "Aabb".to_string();
    let output = "bbAa".to_string();
    assert!(contains_all(&output, &frequency_sort(input)));
}

fn contains_all(a: &str, b: &str) -> bool {
    a.chars().all(|x| b.contains(x))
}
