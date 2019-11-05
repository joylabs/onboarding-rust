use onboarding_rust::week2::exercise14::first_uniq_char;

#[test]
fn test_week2_exercise14_leetcode() {
    let input = "leetcode".to_string();
    assert_eq!(0, first_uniq_char(input));
}

#[test]
fn test_week2_exercise14_loveleetcode() {
    let input = "loveleetcode".to_string();
    assert_eq!(2, first_uniq_char(input));
}

#[test]
fn test_week2_exercise14_loveit() {
    let input = "loveit".to_string();
    assert_eq!(0, first_uniq_char(input));
}

#[test]
fn test_week2_exercise14_loveitloveit() {
    let input = "loveitloveit".to_string();
    assert_eq!(-1, first_uniq_char(input));
}

#[test]
fn test_week2_exercise14_z() {
    let input = "z".to_string();
    assert_eq!(0, first_uniq_char(input));
}
