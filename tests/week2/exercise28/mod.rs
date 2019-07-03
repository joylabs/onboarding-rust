use onboarding_rust::week2::exercise28::distribute_candies;

#[test]
fn one_distribute_candies() {
    let candies = vec![1, 1, 2, 2, 3, 3];
    let output = 3;
    assert_eq!(output, distribute_candies(candies));
}

#[test]
fn two_distribute_candies() {
    let candies = vec![1, 2, 3, 4, 5, 6];
    let output = 3;
    assert_eq!(output, distribute_candies(candies));
}

#[test]
fn three_distribute_candies() {
    let candies = vec![1, 1, 2, 3];
    let output = 2;
    assert_eq!(output, distribute_candies(candies));
}