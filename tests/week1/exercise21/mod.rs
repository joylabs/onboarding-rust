use onboarding_rust::week1::exercise21::exercise21;

#[test]
fn test_1_week1_exercise21() {
    let a = "anagram";
    let b = "nagaram";
    assert_eq!(true, exercise21(a, b));
}

#[test]
fn test_2_week1_exercise21() {
    let a = "rat";
    let b = "car";
    assert_eq!(false, exercise21(a, b));
}


#[test]
fn test3_week1_exercise21() {
    let a = "conversationalists";
    let b = "conservationalists";
    assert_eq!(true, exercise21(a, b));
}