use onboarding_rust::week2::exercise43::longest_word;

#[test]
fn one_longest_word() {
    let words = vec![
        "w".to_string(),
        "wo".to_string(),
        "wor".to_string(),
        "worl".to_string(),
        "world".to_string(),
    ];
    let output = "world".to_string();
    assert_eq!(output, longest_word(words));
}

#[test]
fn two_longest_word() {
    let words = vec![
        "a".to_string(),
        "banana".to_string(),
        "app".to_string(),
        "appl".to_string(),
        "ap".to_string(),
        "apply".to_string(),
        "apple".to_string(),
    ];
    let output = "apple".to_string();
    assert_eq!(output, longest_word(words));
}

#[test]
fn three_longest_word() {
    let words = vec![
        "m".to_string(),
        "mo".to_string(),
        "moc".to_string(),
        "moch".to_string(),
        "mocha".to_string(),
        "l".to_string(),
        "la".to_string(),
        "lat".to_string(),
        "latt".to_string(),
        "latte".to_string(),
        "c".to_string(),
        "ca".to_string(),
        "cat".to_string(),
    ];
    let output = "latte";
    assert_eq!(output, longest_word(words));
}

#[test]
fn four() {
    let words = vec![
        "yo".to_string(),
        "ew".to_string(),
        "fc".to_string(),
        "zrc".to_string(),
        "yodn".to_string(),
        "fcm".to_string(),
        "qm".to_string(),
        "qmo".to_string(),
        "fcmz".to_string(),
        "z".to_string(),
        "ewq".to_string(),
        "yod".to_string(),
        "ewqz".to_string(),
        "y".to_string(),
    ];
    let output = "yodn".to_string();
    assert_eq!(output, longest_word(words));
}