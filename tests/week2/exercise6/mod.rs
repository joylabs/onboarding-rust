use onboarding_rust::week2::exercise6::group_eq_strings;

#[test]
fn test_week1_exercise9_example1() {
    let input = vec!["a", "b", "c", "a", "c", "c"];
    let expected = 3;
    assert_eq!(expected, group_eq_strings(input));
}
#[test]
fn test_week1_exercise9_example2() {
    let input = vec!["aa","bb","ab","ba"];
    let expected = 4;
    assert_eq!(expected, group_eq_strings(input));
}
#[test]
fn test_week1_exercise9_example3() {
    let input = vec!["abc","acb","bac","bca","cab","cba"];
    let expected = 3;
    assert_eq!(expected, group_eq_strings(input));
}
#[test]
fn test_week1_exercise9_example4() {
    let input = vec!["abcd","cdab","adcb","cbad"];
    let expected = 1;
    assert_eq!(expected, group_eq_strings(input));
}
