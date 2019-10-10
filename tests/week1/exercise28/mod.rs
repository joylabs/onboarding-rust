use onboarding_rust::week1::exercise28::distribute_candies;

#[test]
fn test_distribute_candies_three_kinds() {
    let input = vec![1, 1, 2, 2, 3, 3];
    let output = 3;
    assert_eq!(output, distribute_candies(input));
}

#[test]
fn test_distribute_candies_four_kinds() {
    let input = vec![1, 1, 1, 2, 3, 4];
    let output = 3;
    assert_eq!(output, distribute_candies(input));
}

#[test]
fn test_distribute_candies_one_kind() {
    let input = vec![1, 1, 1, 1];
    let output = 1;
    assert_eq!(output, distribute_candies(input));
}