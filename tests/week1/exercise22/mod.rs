use onboarding_rust::week1::exercise22::number_of_islands;

#[test]
fn test_week1_exercise22_example1() {
    let input = vec![
        vec!['1', '1', '1', '1', '0'],
        vec!['1', '1', '0', '1', '0'],
        vec!['1', '1', '0', '0', '0'],
        vec!['0', '0', '0', '0', '0'],
    ];
    let expected = 1;
    assert_eq!(expected, number_of_islands(input));
}

#[test]
fn test_week1_exercise22_example2() {
    let input = vec![
        vec!['1', '1', '0', '0', '0'],
        vec!['1', '1', '0', '0', '0'],
        vec!['0', '0', '1', '0', '0'],
        vec!['0', '0', '0', '1', '1'],
    ];
    let expected = 3;
    assert_eq!(expected, number_of_islands(input));
}
#[test]
fn test_week1_exercise22_example3() {
    let input = vec![
        vec!['1', '1', '0', '1', '0'],
        vec!['1', '1', '1', '0', '1'],
        vec!['0', '0', '1', '1', '1'],
        vec!['0', '0', '0', '1', '0'],
    ];
    let expected = 2;
    assert_eq!(expected, number_of_islands(input));
}