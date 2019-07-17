use onboarding_rust::week2::exercise35::find_anagrams;

#[test]
fn find_all_anagrams_one() {
    let s = "cbaebabacd".to_string();
    let p = "abc".to_string();
    let output = vec![0, 6];
    assert_eq!(output, find_anagrams(s, p));
}

#[test]
fn find_all_anagrams_two() {
    let s = "abab".to_string();
    let p = "ab".to_string();
    let output = vec![0, 6];
    assert_eq!(output, find_anagrams(s, p));
}
