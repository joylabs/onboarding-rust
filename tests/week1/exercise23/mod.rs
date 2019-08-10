use onboarding_rust::week1::exercise23::surrounded_regions;

#[test]
fn test_week1_surrounded_regions_board_one() {
    let mut input = vec![vec!['X', 'X', 'X', 'X'], vec!['X', 'O', 'O', 'X'], vec!['X', 'X', 'O', 'X'], vec!['X', 'O', 'X', 'X']];
    let expected = vec![vec!['X', 'X', 'X', 'X'], vec!['X', 'X', 'X', 'X'], vec!['X', 'X', 'X', 'X'], vec!['X', 'O', 'X', 'X']];
    surrounded_regions(&mut input);
    assert_eq!(expected, input);
}

#[test]
fn test_week1_surrounded_regions_board_no_changes() {
    let mut input = vec![vec!['X', 'O', 'X'], vec!['O', 'X', 'O'], vec!['X', 'O', 'X']];
    let expected = vec![vec!['X', 'O', 'X'], vec!['O', 'X', 'O'], vec!['X', 'O', 'X']];
    surrounded_regions(&mut input);
    assert_eq!(expected, input);
}