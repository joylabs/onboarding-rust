use onboarding_rust::week2::exercise31::isomorphic_string;

#[test]
fn one_isomorphic() {
    println!("intenti q");
    let s = "edd".to_string();
    let t = "agg".to_string();
    assert!(isomorphic_string(s, t));
}

#[test]
fn two_isomorphic() {
    let s = "foo".to_string();
    let t = "bar".to_string();

    assert!(!isomorphic_string(s, t));
}

#[test]
fn three_isomorphic() {
    let s = "title".to_string();
    let t = "paper".to_string();
    assert!(isomorphic_string(s, t));
}

#[test]
fn four_isomorphic() {
    let s = "happy_birthday".to_string();
    let t = "detta_pirodzea".to_string();
    assert!(isomorphic_string(s, t));
}