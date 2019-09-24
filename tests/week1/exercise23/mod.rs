use onboarding_rust::week1::exercise23::capture_surrounded_regions;

#[test]
fn test_week1_exercise23_ex1() {
    let mut input = vec![
        vec!['X', 'X', 'X', 'X'],
        vec!['X', 'O', 'O', 'X'],
        vec!['X', 'X', 'O', 'X'],
        vec!['X', 'O', 'X', 'X'],
    ];
    let expected = vec![
        vec!['X', 'X', 'X', 'X'],
        vec!['X', 'X', 'X', 'X'],
        vec!['X', 'X', 'X', 'X'],
        vec!['X', 'O', 'X', 'X'],
    ];
    capture_surrounded_regions(&mut input);
    assert_eq!(expected, input);
}

#[test]
fn test_week1_exercise23_ex2() {
    let mut input = vec![
        vec!['X', 'O', 'X', 'X'],
        vec!['X', 'O', 'X', 'X'],
        vec!['X', 'X', 'O', 'X'],
        vec!['X', 'O', 'X', 'X'],
    ];
    let expected = vec![
        vec!['X', 'O', 'X', 'X'],
        vec!['X', 'O', 'X', 'X'],
        vec!['X', 'X', 'X', 'X'],
        vec!['X', 'O', 'X', 'X'],
    ];
    capture_surrounded_regions(&mut input);
    assert_eq!(expected, input);
}

#[test]
fn test_week1_exercise23_ex3() {
    let mut input = vec![
        vec!['X', 'O', 'X', 'O', 'X', 'O'],
        vec!['O', 'X', 'O', 'X', 'O', 'X'],
        vec!['X', 'O', 'X', 'O', 'X', 'O'],
        vec!['O', 'X', 'O', 'X', 'O', 'X'],
    ];
    let expected = vec![
        vec!['X', 'O', 'X', 'O', 'X', 'O'],
        vec!['O', 'X', 'X', 'X', 'X', 'X'],
        vec!['X', 'X', 'X', 'X', 'X', 'O'],
        vec!['O', 'X', 'O', 'X', 'O', 'X'],
    ];
    capture_surrounded_regions(&mut input);
    assert_eq!(expected, input);
}
