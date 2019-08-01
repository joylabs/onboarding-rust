use onboarding_rust::week2::exercise21::longest_word;

#[test]
fn test_week2_exercise21_example1() {
    let input = vec!["w".to_string(),"wo".to_string(),"wor".to_string(),"worl".to_string(), "world".to_string()];
    let expected = "world".to_string();
    assert_eq!(expected, longest_word(input));
}

#[test]
fn test_week2_exercise21_example2() {
    let input = vec!["a".to_string(), "banana".to_string(), "app".to_string(), "appl".to_string(), "ap".to_string(), "apply".to_string(), "apple".to_string()];
    let expected = "apple".to_string();
    assert_eq!(expected, longest_word(input));
}

#[test]
fn test_week2_exercise21_example3() {
    let input = vec!["b".to_string(),"br".to_string(),"bre".to_string(),"brea".to_string(),"break".to_string(),"breakf".to_string(),"breakfa".to_string(),"breakfas".to_string(),"breakfast".to_string(),"l".to_string(),"lu".to_string(),"lun".to_string(),"lunc".to_string(),"lunch".to_string(),"d".to_string(),"di".to_string(),"din".to_string(),"dinn".to_string(),"dinne".to_string(),"dinner".to_string()];
    let expected = "breakfast".to_string();
    assert_eq!(expected, longest_word(input));
}

#[test]
fn test_week2_exercise21_example4() {
    let input = vec!["t".to_string(),"ti".to_string(),"tig".to_string(),"tige".to_string(),"tiger".to_string(),"e".to_string(),"en".to_string(),"eng".to_string(),"engl".to_string(),"engli".to_string(),"englis".to_string(),"english".to_string(),"h".to_string(),"hi".to_string(),"his".to_string(),"hist".to_string(),"histo".to_string(),"histor".to_string(),"history".to_string()];
    let expected = "english".to_string();
    assert_eq!(expected, longest_word(input));
}
