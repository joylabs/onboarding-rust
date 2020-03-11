use onboarding_rust::week2::exercise9::is_isomorphic;

#[test]
fn test_week2_exercise8_example1() {
    let s = "egg".to_string();
    let t = "add".to_string();

    let expected = true;

    assert_eq!(expected, is_isomorphic(s, t));
}

#[test]
fn test_week2_exercise8_example2() {
    let s = "foo".to_string();
    let t = "bar".to_string();

    let expected = false;

    assert_eq!(expected, is_isomorphic(s, t));
}

#[test]
fn test_week2_exercise8_example3() {
    let s = "paper".to_string();
    let t = "title".to_string();

    let expected = true;

    assert_eq!(expected, is_isomorphic(s, t));
}

#[test]
fn test_week2_exercise8_example4() {
    let s = "egg".to_string();
    let t = "dad".to_string();

    let expected = false;

    assert_eq!(expected, is_isomorphic(s, t));
}
