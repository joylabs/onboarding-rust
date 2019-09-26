use onboarding_rust::week3::exercise14::is_match;

#[test]
fn test_week3_exercise14_example1() {
    let s = "aa".to_owned();
    let p = "a".to_owned();
    let expected = false;
    assert_eq!(expected, is_match(s, p));
}

#[test]
fn test_week3_exercise14_example2() {
    let s = "aa".to_owned();
    let p = "a*".to_owned();
    let expected = true;
    assert_eq!(expected, is_match(s, p));
}

#[test]
fn test_week3_exercise14_example3() {
    let s = "ab".to_owned();
    let p = ".*".to_owned();
    let expected = true;
    assert_eq!(expected, is_match(s, p));
}

#[test]
fn test_week3_exercise14_example4() {
    let s = "aab".to_owned();
    let p = "c*a*b".to_owned();
    let expected = true;
    assert_eq!(expected, is_match(s, p));
}

#[test]
fn test_week3_exercise14_example5() {
    let s = "mississippi".to_owned();
    let p = "mis*is*p*.".to_owned();
    let expected = false;
    assert_eq!(expected, is_match(s, p));
}
