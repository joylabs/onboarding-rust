use onboarding_rust::week1::exercise10::exercise10;

#[test]
fn test_1_week1_exercise10() {
    let mut a = vec!["h","e","l","l","o"];
    let b = vec!["o","l","l","e","h"]; 
    exercise10(&mut a);
    assert_eq!(b,a);
}

#[test]
fn test_2_week1_exercise10() {
    let mut a = vec!["H","a","n","n","a","h"];
    let b = vec!["h","a","n","n","a","H"]; 
    exercise10(&mut a);
    assert_eq!(b,a);
}