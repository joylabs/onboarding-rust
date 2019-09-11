use onboarding_rust::week2::exercise23::find_words;

#[test]
fn test_week2_exercise23_example1() {
    let board = vec![
        vec!['o', 'a', 'a', 'n'],
        vec!['e', 't', 'a', 'e'],
        vec!['i', 'h', 'k', 'r'],
        vec!['i', 'f', 'l', 'v'],
    ];
    let words = vec![
        "oath".to_string(),
        "pea".to_string(),
        "eat".to_string(),
        "rain".to_string(),
    ];
    let expected = vec!["oath".to_string(), "eat".to_string()];
    assert_eq!(expected, find_words(board, words));
}

#[test]
fn test_week2_exercise23_example2() {
    let board = vec![];
    let words = vec![
        "oath".to_string(),
        "pea".to_string(),
        "eat".to_string(),
        "rain".to_string(),
    ];
    let expected: Vec<String> = vec![];
    assert_eq!(expected, find_words(board, words));
}

#[test]
fn test_week2_exercise23_example3() {
    let board = vec![vec!['a', 'b'], vec!['a', 'a']];
    let words = vec![
        "aba".to_string(),
        "baa".to_string(),
        "bab".to_string(),
        "aaab".to_string(),
        "aaa".to_string(),
        "aaaa".to_string(),
        "aaba".to_string(),
    ];
    let expected = vec![
        "aba".to_string(),
        "baa".to_string(),
        "aaab".to_string(),
        "aaa".to_string(),
        "aaba".to_string(),
    ];
    assert_eq!(expected, find_words(board, words));
}
