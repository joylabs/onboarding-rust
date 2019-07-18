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

#[test]
fn test_week2_exercise13_example3() {
    let s = "baa";
    let p = "aa";
    let expected = vec![1];

    assert_eq!(expected, find_anagrams(s, p));
}

#[test]
fn test_week2_exercise13_example4() {
    let s = "ababababab";
    let p = "aab";
    let expected = vec![0, 2, 4, 6];

    assert_eq!(expected, find_anagrams(s, p));
}

#[test]
fn test_week2_exercise13_example5() {
    let s = "acdcaeccde";
    let p = "c";
    let expected = vec![1, 3, 6, 7];

    assert_eq!(expected, find_anagrams(s, p));
}

#[test]
fn test_week2_exercise13_example6() {
    let s = "aecbabedce";
    let p = "a";
    let expected = vec![0, 4];

    assert_eq!(expected, find_anagrams(s, p));
}