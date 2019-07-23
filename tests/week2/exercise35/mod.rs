use onboarding_rust::week2::exercise35::find_anagrams;

#[test]
fn find_all_anagrams_one() {
    let s = "cbaebabacd";
    let p = "abc";
    let output = vec![0, 6];
    assert_eq!(output, find_anagrams(s, p));
}

#[test]
fn find_all_anagrams_two() {
    let s = "abab";
    let p = "ab";
    let output = vec![0, 1, 2];
    assert_eq!(output, find_anagrams(s, p));
}

#[test]
fn find_all_anagrams_three() {
    let s = "aa";
    let p = "bb";
    let output: Vec<i32> = vec![];
    assert_eq!(output, find_anagrams(s, p));
}
