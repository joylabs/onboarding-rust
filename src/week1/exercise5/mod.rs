pub fn fizz_buzz(n: i32) -> Vec<String> {
    (1..=n)
        .map(|number| {
            if number % 3 == 0 && number % 5 == 0 {
                return "FizzBuzz".to_owned();
            } else if number % 3 == 0 {
                return "Fizz".to_owned();
            } else if number % 5 == 0 {
                return "Buzz".to_owned();
            }
            number.to_string()
        })
        .collect()
}