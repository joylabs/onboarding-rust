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

#[test]
fn test_week3_exercise14_example6() {
    let s = "".to_owned();
    let p = ".*".to_owned();
    let expected = true;
    assert_eq!(expected, is_match(s, p));
}

#[test]
fn test_week3_exercise14_example7() {
    let s = "".to_owned();
    let p = ".".to_owned();
    let expected = false;
    assert_eq!(expected, is_match(s, p));
}

#[test]
fn test_week3_exercise14_example8() {
    let s = "".to_owned();
    let p = "".to_owned();
    let expected = true;
    assert_eq!(expected, is_match(s, p));
}

#[test]
fn test_week3_exercise14_example9() {
    let s = "a".to_owned();
    let p = "".to_owned();
    let expected = false;
    assert_eq!(expected, is_match(s, p));
}

#[test]
fn test_week3_exercise14_example10() {
    let s = "abcd".to_owned();
    let p = "d*".to_owned();
    let expected = false;
    assert_eq!(expected, is_match(s, p));
}

#[test]
fn test_week3_exercise14_example11() {
    let s = "ab".to_owned();
    let p = ".*c".to_owned();
    let expected = false;
    assert_eq!(expected, is_match(s, p));
}

#[test]
fn test_week3_exercise14_example12() {
    let s = "aaa".to_owned();
    let p = "a*a".to_owned();
    let expected = true;
    assert_eq!(expected, is_match(s, p));
}

#[test]
fn test_week3_exercise14_example13() {
    let s = "aaa".to_owned();
    let p = "ab*a*c*a".to_owned();
    let expected = true;
    assert_eq!(expected, is_match(s, p));
}

#[test]
fn test_week3_exercise14_example14() {
    let s = "aaa".to_owned();
    let p = "aaaa".to_owned();
    let expected = false;
    assert_eq!(expected, is_match(s, p));
}

#[test]
fn test_week3_exercise14_example15() {
    let s = "a".to_owned();
    let p = "ab*a".to_owned();
    let expected = false;
    assert_eq!(expected, is_match(s, p));
}

#[test]
fn test_week3_exercise14_example16() {
    let s = "a".to_owned();
    let p = ".*..a*".to_owned();
    let expected = false;
    assert_eq!(expected, is_match(s, p));
}

#[test]
fn test_week3_exercise14_example17() {
    let s = "ab".to_owned();
    let p = ".*..".to_owned();
    let expected = true;
    assert_eq!(expected, is_match(s, p));
}

#[test]
fn test_week3_exercise14_example18() {
    let s = "aab".to_owned();
    let p = "a*c*b".to_owned();
    let expected = true;
    assert_eq!(expected, is_match(s, p));
}

#[test]
fn test_week3_exercise14_example19() {
    let s = "aasdfasdfasdfasdfas".to_owned();
    let p = "aasdf.*asdf.*asdf.*asdf.*s".to_owned();
    let expected = true;
    assert_eq!(expected, is_match(s, p));
}

#[test]
fn test_week3_exercise14_example20() {
    let s = "".to_owned();
    let p = "c*c*".to_owned();
    let expected = true;
    assert_eq!(expected, is_match(s, p));
}

#[test]
fn test_week3_exercise14_example21() {
    let s = "ab".to_owned();
    let p = ".*b*".to_owned();
    let expected = true;
    assert_eq!(expected, is_match(s, p));
}

#[test]
fn test_week3_exercise14_example22() {
    let s = "a".to_owned();
    let p = ".*..".to_owned();
    let expected = false;
    assert_eq!(expected, is_match(s, p));
}

#[test]
fn test_week3_exercise14_example23() {
    let s = "a".to_owned();
    let p = "b*.c".to_owned();
    let expected = false;
    assert_eq!(expected, is_match(s, p));
}

#[test]
fn test_week3_exercise14_example24() {
    let s = "abbabaaaaaaacaa".to_owned();
    let p = "a*.*b.a.*c*b*a*c*".to_owned();
    let expected = true;
    assert_eq!(expected, is_match(s, p));
}