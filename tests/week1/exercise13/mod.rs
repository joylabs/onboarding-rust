use onboarding_rust::week1::exercise13::exercise13;

#[test]
fn test_1_week1_exercise13() {
    let a = "hello".to_string();
    let b = "holle".to_string();
    assert_eq!(b, exercise13(a));
}

#[test]
fn test_2_week1_exercise13() {
    let a = "leetcode".to_string();
    let b = "leotcede".to_string();
    assert_eq!(b, exercise13(a));
}