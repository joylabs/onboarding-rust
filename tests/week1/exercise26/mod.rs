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

#[test]
fn test_2_exercise_26() {
    let a = vec![
        "mgic".to_string(),
        "stdi".to_string(),
        "stdu".to_string(),
        "msge".to_string(),
    ];
    let b = 4;
    assert_eq!(b, unique_morse_representations(a));
}


#[test]
fn test_3_exercise_26() {
    let a = vec![
        "acbe".to_string(),
        "pnth".to_string(),
        "stdu".to_string(),
        "msge".to_string(),
    ];
    let b = 3;
    assert_eq!(b, unique_morse_representations(a));
}