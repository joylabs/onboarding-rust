use onboarding_rust::week1::exercise5::fizz_buzz;

#[test]
fn test_week1_exercise5_example1() {

    let input = 15;
    let expected = vec![
        "1", "2", "Fizz", "4", "Buzz", "Fizz", "7", "8", "Fizz", "Buzz", "11", "Fizz", "13", "14",
        "FizzBuzz",
    ];
    assert_eq!(expected, fizz_buzz(input));
}
