use onboarding_rust::week2::exercise9::is_isomorphic;

#[test]
fn test_week2_exercise9_example1() {
    let s = String::from("egg");
    let t = String::from("add");
    let expected = true;
    assert_eq!(expected, is_isomorphic(s, t));
}

#[test]
fn test_week2_exercise9_example2() {
    let s = String::from("foo");
    let t = String::from("bar");
    let expected = false;
    assert_eq!(expected, is_isomorphic(s, t));
}

#[test]
fn test_week2_exercise9_example3() {
    let s = String::from("paper");
    let t = String::from("title");
    let expected = true;
    assert_eq!(expected, is_isomorphic(s, t));
}