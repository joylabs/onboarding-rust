use onboarding_rust::week1::exercise32::is_isomorphic;

#[test]
fn test_is_isomorphic() {
    let s = String::from("egg");
    let t = String::from("add");
    let output = true;
    assert_eq!(output, is_isomorphic(s, t));
}

#[test]
fn test_is_not_isomorphic() {
    let s = String::from("foo");
    let t = String::from("bar");
    let output = false;
    assert_eq!(output, is_isomorphic(s, t));
}
