use onboarding_rust::week1::exercise21::exercise21;

#[test]
fn test_1_week1_exercise21() {
    let a = "anagram".to_string();
    let b = "nagaram".to_string();
    assert_eq!(true, exercise21(a, b));
}

#[test]
fn test_2_week1_exercise21() {
    let a = "rat".to_string();
    let b = "car".to_string();
    assert_eq!(false, exercise21(a, b));
}


#[test]
fn test3_week1_exercise21() {
    let a = "conversationalists".to_string();
    let b = "conservationalists".to_string();
    assert_eq!(true, exercise21(a, b));
}