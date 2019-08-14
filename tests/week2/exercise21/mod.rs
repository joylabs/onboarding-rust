use onboarding_rust::week2::exercise21::longest_word;

#[test]
fn test_week2_exercise21_example1() {
    let input = vec![
        "w".to_string(),
        "wo".to_string(),
        "wor".to_string(),
        "worl".to_string(),
        "world".to_string(),
    ];
    let expected = "world".to_string();
    assert_eq!(expected, longest_word(input));
}

#[test]

fn test_week2_exercise21_example2() {
    let input = vec![
        "a".to_string(),
        "banana".to_string(),
        "app".to_string(),
        "appl".to_string(),
        "ap".to_string(),
        "apply".to_string(),
        "apple".to_string(),
    ];
    let expected = "apple".to_string();
    assert_eq!(expected, longest_word(input));
}

#[test]

fn test_week2_exercise21_example3() {
    let input = vec![
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
    let expected = "latte";
    assert_eq!(expected, longest_word(input));
}

#[test]

fn test_week2_exercise21_example4() {
    let input = vec![
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
    let expected = "yodn".to_string();
    assert_eq!(expected, longest_word(input));
}

#[test]
fn test_week2_exercise21_example5() {
    let input = vec![
        "k".to_string(),
        "lg".to_string(),
        "it".to_string(),
        "oidd".to_string(),
        "oid".to_string(),
        "oiddm".to_string(),
        "kfk".to_string(),
        "y".to_string(),
        "mw".to_string(),
        "kf".to_string(),
        "l".to_string(),
        "o".to_string(),
        "mwaqz".to_string(),
        "oi".to_string(),
        "ych".to_string(),
        "m".to_string(),
        "mwa".to_string(),
    ];
    let expected = "oiddm".to_string();
    assert_eq!(expected, longest_word(input));
}

#[test]
fn test_week2_exercise21_example6() {
    let input = vec![
        "rac".to_string(),
        "rs".to_string(),
        "ra".to_string(),
        "on".to_string(),
        "r".to_string(),
        "otif".to_string(),
        "o".to_string(),
        "onpdu".to_string(),
        "rsf".to_string(),
        "rs".to_string(),
        "ot".to_string(),
        "oti".to_string(),
        "racy".to_string(),
        "onpd".to_string(),
    ];
    let expected = "otif".to_string();
    assert_eq!(expected, longest_word(input));
}
