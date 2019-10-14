use onboarding_rust::week2::exercise13::find_anagrams;

#[test]
fn test_week2_exercise13_example1() {
    let input_s = "cbaebabacd".to_string();
    let input_p = "acb".to_string();
    let result = vec![0, 6];
    assert_eq!(result, find_anagrams(input_s, input_p));
}

#[test]
fn test_week2_exercise13_example2() {
    let input_s = "abab".to_string();
    let input_p = "ab".to_string();
    let result = vec![0, 1, 2];
    assert_eq!(result, find_anagrams(input_s, input_p));
}

#[test]
fn test_week2_exercise13_example3() {
    let input_s = "baa".to_string();
    let input_p = "aa".to_string();
    let expected = vec![1];

    assert_eq!(expected, find_anagrams(input_s, input_p));
}

#[test]
fn test_week2_exercise13_example4() {
    let input_s = "ababababab".to_string();
    let input_p = "aab".to_string();
    let expected = vec![0, 2, 4, 6];

    assert_eq!(expected, find_anagrams(input_s, input_p));
}

#[test]
fn test_week2_exercise13_example5() {
    let input_s = "acdcaeccde".to_string();
    let input_p = "c".to_string();
    let expected = vec![1, 3, 6, 7];

    assert_eq!(expected, find_anagrams(input_s, input_p));
}

#[test]
fn test_week2_exercise13_example6() {
    let input_s = "aecbabedce".to_string();
    let input_p = "a".to_string();
    let expected = vec![0, 4];

    assert_eq!(expected, find_anagrams(input_s, input_p));
}
