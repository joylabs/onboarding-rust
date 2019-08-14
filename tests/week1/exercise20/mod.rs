use onboarding_rust::week1::exercise20::valid_anagram;

#[test]
fn test_week1_exercise20_example1() {
    let input_1 = "anagram".to_string();
    let input_2 = "nagaram".to_string();
    let expected = true;
    assert_eq!(expected, valid_anagram(input_1, input_2));
}

#[test]
fn test_week1_exercise20_example2() {
    let input_1 = "rat".to_string();
    let input_2 = "car".to_string();
    let expected = false;
    assert_eq!(expected, valid_anagram(input_1, input_2));
}

#[test]
fn test_week1_exercise20_example3() {
    let input_1 = "´ra&t".to_string();
    let input_2 = "&ta´r".to_string();
    let expected = true;
    assert_eq!(expected, valid_anagram(input_1, input_2));
}

#[test]
fn test_week1_exercise20_example4() {
    let input_1 = "Здравствуйте".to_string();
    let input_2 = "Здрйвствауте".to_string();
    let expected = true;
    assert_eq!(expected, valid_anagram(input_1, input_2));
}

#[test]
fn test_week1_exercise20_example5() {
    let input_1 = "dad".to_string();
    let input_2 = "da".to_string();
    let expected = false;
    assert_eq!(expected, valid_anagram(input_1, input_2));
}

#[test]
fn test_week1_exercise20_example6() {
    let input_1 = "da".to_string();
    let input_2 = "dad".to_string();
    let expected = false;
    assert_eq!(expected, valid_anagram(input_1, input_2));
}
