use onboarding_rust::week2::exercise27::uncommon_words;


#[test]
fn one_uncommon_word() {
    let a = "this apple is sweet".to_string();
    let b = "this apple is sour".to_string();
    let output = vec!["sweet".to_string(), "sour".to_string()];

    assert!(contains_all(output, uncommon_words(a, b)));
}

#[test]
fn two_uncommon_word() {
    let a = "the weather is hot and warm".to_string();
    let b = "today the weather is cool".to_string();
    let output = vec![
        "hot".to_string(),
        "and".to_string(),
        "warm".to_string(),
        "today".to_string(),
        "cool".to_string(),
    ];
    assert!(contains_all(output, uncommon_words(a, b)));
}

#[test]
fn three_uncommon_word() {
    let a = "apple apple".to_string();
    let b = "banana".to_string();
    let output = vec!["banana".to_string()];
    assert!(contains_all(output, uncommon_words(a, b)));
}

fn contains_all(a: Vec<String>, b: Vec<String>) -> bool {
    a.len() == b.len() && a.iter().all(|x| b.contains(x))
}