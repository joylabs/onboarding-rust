use onboarding_rust::week2::exercise31::isomorphic_string;

#[test]
fn one_isomorphic() {
    let s = "agged".to_string();
    let t = "sdded".to_string();
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
    let s = "sssshhjhjjritbdjsuwiedbcnjxjsjeeesehebaksldjolkerbs".to_string();
    let t = "ppppiijhffritbdjsuwiedbcnjxjsjlllrhxtabstojlkymjith".to_string();

    assert!(isomorphic_string(s, t));
}