use onboarding_rust::week2::exercise9::is_isomorphic;

#[test]
fn test_week2_exercise9_egg() {
    let input_s = "egg".to_string();
    let input_t = "add".to_string();
    let expected = true;
    assert_eq!(expected, is_isomorphic(input_s, input_t));
}

#[test]
fn test_week2_exercise9_foo() {
    let input_s = "foo".to_string();
    let input_t = "bar".to_string();
    let expected = false;
    assert_eq!(expected, is_isomorphic(input_s, input_t));
}

#[test]
fn test_week2_exercise9_paper() {
    let input_s = "paper".to_string();
    let input_t = "title".to_string();
    let expected = true;
    assert_eq!(expected, is_isomorphic(input_s, input_t));
}

#[test]
fn test_week2_exercise9_ab() {
    let input_s = "ab".to_string();
    let input_t = "aa".to_string();
    let expected = false;
    assert_eq!(expected, is_isomorphic(input_s, input_t));
}
