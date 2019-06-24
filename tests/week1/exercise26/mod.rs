use onboarding_rust::week1::exercise26::unique_morse_representations;

#[test]
fn test_1_exercise_26() {
    let a = vec![
        "gin".to_string(),
        "zen".to_string(),
        "gig".to_string(),
        "msg".to_string(),
    ];
    let b = 2;
    assert_eq!(b, unique_morse_representations(a));
}

