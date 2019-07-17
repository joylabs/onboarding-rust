use onboarding_rust::week2::exercise13::find_anagrams;

#[test]
fn test_week2_exercise13_example1() {
    let s = "cbaebabacd";
    let p = "abc";
    let expected = vec![0, 6];
    
    assert_eq!(expected, find_anagrams(s, p));
}

#[test]
fn test_week2_exercise13_example2() {
    let s = "abab";
    let p = "ab";
    let expected = vec![0, 1, 2];
    
    assert_eq!(expected, find_anagrams(s, p));
}