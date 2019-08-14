pub fn fizz_buzz(input: i32) -> Vec<String> {
    (1..=input).map(eveluate).collect()
}

fn eveluate(number: i32) -> String {
    if number % 3 == 0 && number % 5 == 0 {
        "FizzBuzz".to_owned()
    } else if number % 3 == 0 {
        "Fizz".to_owned()
    } else if number % 5 == 0 {
        "Buzz".to_owned()
    } else {
        number.to_string()
    }
}