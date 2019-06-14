use onboarding_rust::week1::exercise9::reverse_string;

#[test]
fn test_week1_exercise9_example1() {
    
    let mut input = vec!["h","e","l","l","o"];
    let expected = vec!["o","l","l","e","h"];
    reverse_string(&mut input);
    assert_eq!(expected, input);
}

#[test]
fn test_week1_exercise9_example2() {
    
    let mut input = vec!["H","a","n","n","a","h"];
    let expected = vec!["h","a","n","n","a","H"];
    reverse_string(&mut input);
    assert_eq!(expected, input);
}