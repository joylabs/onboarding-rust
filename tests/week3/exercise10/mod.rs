use onboarding_rust::week3::exercise10::max_profit;

#[test]
fn test_best_time_to_buy_one() {
    let input = vec![7, 1, 5, 3, 6, 4];
    let output = 5;
    assert_eq!(output, max_profit(input));
}

#[test]
fn test_best_time_to_buy_two() {
    let input = vec![7, 6, 4, 3, 1];
    let output = 0;
    assert_eq!(output, max_profit(input));
}

#[test]
fn test_best_time_to_buy_three() {
    let input = vec![2, 4, 1];
    let output = 2;
    assert_eq!(output, max_profit(input));
}
