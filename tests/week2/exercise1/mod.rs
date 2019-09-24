use onboarding_rust::week2::exercise1::jewels_in_stones;

#[test]
fn test_week2_exercise1_example1() {
    let jewels = "aA".to_string();
    let stones = "aAAbbbb".to_string();

    let expected = 3;

    assert_eq!(expected, jewels_in_stones(jewels, stones));
}

#[test]
fn test_week2_exercise1_example2() {
    let jewels = "z".to_string();
    let stones = "ZZ".to_string();

    let expected = 0;

    assert_eq!(expected, jewels_in_stones(jewels, stones));
}

#[test]
fn test_week2_exercise1_example3() {
    let jewels = "hOe".to_string();
    let stones = "abshhiOSJENCJesfe".to_string();

    let expected = 5;

    assert_eq!(expected, jewels_in_stones(jewels, stones));
}
