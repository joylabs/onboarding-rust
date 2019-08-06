use onboarding_rust::week1::exercise22::number_of_islands;

#[test]
fn test_week1_number_of_islands_one() {
    let input = vec![
        vec!['1', '1', '1', '1', '0'],
        vec!['1', '1', '0', '1', '0'],
        vec!['1', '1', '0', '0', '0'],
        vec!['0', '0', '0', '0', '0'],
    ];
    let output = 1;
    assert_eq!(output, number_of_islands(input));
}

#[test]
fn test_week1_number_of_islands_three() {
    let input = vec![
        vec!['1', '1', '0', '0', '0'],
        vec!['1', '1', '0', '0', '0'],
        vec!['0', '0', '1', '0', '0'],
        vec!['0', '0', '0', '1', '1'],
    ];
    let output = 3;
    assert_eq!(output, number_of_islands(input));
}

#[test]
fn test_week1_number_of_islands_one_minimum() {
    let input = vec![vec!['1']];
    let output = 1;
    assert_eq!(output, number_of_islands(input));
}