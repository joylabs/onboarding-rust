use onboarding_rust::week2::exercise46::exist;


#[test]
fn one_test_word_search() {
    let board = vec![
        vec!['A', 'B', 'C', 'E'],
        vec!['S', 'F', 'C', 'S'],
        vec!['A', 'D', 'E', 'E'],
    ];
    let word = "ABCCED".to_string();
    assert!(exist(board, word));
}

#[test]
fn two_test_word_search() {
    let board = vec![
        vec!['C', 'A', 'A'],
        vec!['A', 'A', 'A'],
        vec!['B', 'C', 'D'],
    ];
    let word = "AAB".to_string();
    // let board = vec![vec!['a', 'a']];
    // let word = "aaa".to_string();
    assert!(exist(board, word));
}

#[test]
fn three_test_word_search() {
    let board = vec![
        vec!['A', 'B', 'C', 'E'],
        vec!['S', 'E', 'E', 'S'],
        vec!['A', 'D', 'E', 'E'],
    ];
    let word = "ABCEFSADEESE".to_string();
    assert!(!exist(board, word));
}


// [["A","B","C","E"],["S","E","E","S"],["A","D","E","E"]]
// "ABCEFSADEESE"
