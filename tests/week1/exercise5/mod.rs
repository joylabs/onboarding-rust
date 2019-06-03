use onboarding_rust::week1::exercise5::exercise_5;

#[test]
fn test_week1_exercise5() {
    let v = vec![
        "1", "2", "Fizz", "4", "Buzz", "Fizz", "7", "8", "Fizz", "Buzz", "11", "Fizz", "13", "14",
        "FizzBuzz",
    ];
    let value: i64 = 15;
    assert_eq!(v, exercise_5(value));
}