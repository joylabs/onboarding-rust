use onboarding_rust::week1::exercise23::capture_surrounded_regions;

#[test]
fn test_week1_exercise23_example1() {
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
fn test_week1_exercise23_example2() {
    let mut input = vec![
        vec!['O', 'O', 'O'],
        vec!['O', 'O', 'O'],
        vec!['O', 'O', 'O'],
    ];
    let expected = vec![
        vec!['O', 'O', 'O'],
        vec!['O', 'O', 'O'],
        vec!['O', 'O', 'O'],
    ];
    capture_surrounded_regions(&mut input);
    assert_eq!(expected, input);
}

#[test]
fn test_week1_exercise23_example3() {
    let mut input = vec![
        vec!['O', 'X', 'X', 'O', 'X'],
        vec!['X', 'O', 'O', 'X', 'O'],
        vec!['X', 'O', 'X', 'O', 'X'],
        vec!['O', 'X', 'O', 'O', 'O'],
        vec!['X', 'X', 'O', 'X', 'O'],
    ];
    let expected = vec![
        vec!['O', 'X', 'X', 'O', 'X'],
        vec!['X', 'X', 'X', 'X', 'O'],
        vec!['X', 'X', 'X', 'O', 'X'],
        vec!['O', 'X', 'O', 'O', 'O'],
        vec!['X', 'X', 'O', 'X', 'O'],
    ];
    capture_surrounded_regions(&mut input);
    assert_eq!(expected, input);
}

#[test]
fn test_week1_exercise23_example4() {
    let mut input = vec![
        vec!['O','X','O','O','X','X'],
        vec!['O','X','X','X','O','X'],
        vec!['X','O','O','X','O','O'],
        vec!['X','O','X','X','X','X'],
        vec!['O','O','X','O','X','X'],
        vec!['X','X','O','O','O','O']
    ];
    let expected = vec![
        vec!['O','X','O','O','X','X'],
        vec!['O','X','X','X','O','X'],
        vec!['X','O','O','X','O','O'],
        vec!['X','O','X','X','X','X'],
        vec!['O','O','X','O','X','X'],
        vec!['X','X','O','O','O','O']
    ];
    capture_surrounded_regions(&mut input);
    assert_eq!(expected, input);
}

#[test]
fn test_week1_exercise23_example5() {
    let mut input = vec![
        vec!['O','O','O','O','O','O'],
        vec!['O','X','X','X','X','O'],
        vec!['O','X','O','O','X','O'],
        vec!['O','X','O','O','X','O'],
        vec!['O','X','X','X','X','O'],
        vec!['O','O','O','O','O','O']
    ];
    let expected = vec![
        vec!['O','O','O','O','O','O'],
        vec!['O','X','X','X','X','O'],
        vec!['O','X','X','X','X','O'],
        vec!['O','X','X','X','X','O'],
        vec!['O','X','X','X','X','O'],
        vec!['O','O','O','O','O','O']
    ];
    capture_surrounded_regions(&mut input);
    assert_eq!(expected, input);
}