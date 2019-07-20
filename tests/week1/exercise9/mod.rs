use onboarding_rust::week1::exercise9::reverse_string;

#[test]
fn test_1_week1_exercise10() {
    let mut a = "hello".chars().collect::<Vec<char>>();
    let b = "olleh".chars().collect::<Vec<char>>();
    reverse_string(&mut a);
    assert_eq!(b, a);
}

#[test]
fn test_2_week1_exercise10() {
    let mut a = "Hannah".chars().collect::<Vec<char>>();
    let b = "hannaH".chars().collect::<Vec<char>>();
    reverse_string(&mut a);
    assert_eq!(b, a);
}