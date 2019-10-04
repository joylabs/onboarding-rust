use onboarding_rust::week3::exercise3::find_min_arrow_shots;


#[test]
fn test_number_of_arrows_one() {
    let input = vec![
        vec![10, 16],
        vec![2, 8],
        vec![1, 6],
        vec![7, 12],
        vec![3, 5],
    ];
    let output = 2;
    assert_eq!(output, find_min_arrow_shots(input));
}

#[test]
fn test_number_of_arrows_two() {
    let input = vec![
        vec![9, 12],
        vec![1, 10],
        vec![4, 11],
        vec![8, 12],
        vec![3, 9],
        vec![6, 9],
        vec![6, 7],
    ];
    let output = 2;
    assert_eq!(output, find_min_arrow_shots(input));
}

#[test]
fn test_number_of_arrows_three() {
    let input = vec![vec![-2147483648, 2147483647]];
    let output = 1;
    assert_eq!(output, find_min_arrow_shots(input));
}

//[[-2147483648,2147483647]]
//[[9,12],[1,10],[4,11],[8,12],[3,9],[6,9],[6,7]]