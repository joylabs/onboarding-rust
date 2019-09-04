use onboarding_rust::week1::exercise22::num_islands;

#[test]
fn test_week1_exercise22_one() {
    let input = vec![
        vec!['1', '1', '1', '1', '0'],
        vec!['1', '1', '0', '1', '0'],
        vec!['1', '1', '0', '0', '0'],
        vec!['0', '0', '0', '0', '0'],
    ];
    let output = 1;
    assert_eq!(output, num_islands(input));
}

#[test]
fn test_week1_exercise22_two() {
    let input = vec![
        vec!['1', '1', '0', '0', '0'],
        vec!['1', '1', '0', '0', '0'],
        vec!['0', '0', '1', '0', '0'],
        vec!['0', '0', '0', '1', '1'],
    ];
    let output = 3;
    assert_eq!(output, num_islands(input));
}