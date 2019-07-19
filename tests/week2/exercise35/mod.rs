use onboarding_rust::week2::exercise35::find_anagrams;

#[test]
fn find_all_anagrams_one() {
    let s = "cbaebabacd".to_string();
    let p = "abc";
    let output = vec![0, 6];
    assert_eq!(output, find_anagrams(s, p));
}

#[test]
fn find_all_anagrams_two() {
    let s = "abab".to_string();
    let p = "ab";
    let output = vec![0, 1, 2];
    assert_eq!(output, find_anagrams(s, p));
}

#[test]
fn find_all_anagrams_three() {
    let s = "aa".to_string();
    let p = "bb";
    let output: Vec<i32> = vec![];
    assert_eq!(output, find_anagrams(s, p));
}
