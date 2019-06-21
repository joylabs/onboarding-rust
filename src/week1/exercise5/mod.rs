pub fn fizz_buzz(n: i32) -> Vec<String> {
    let mut output: Vec<String> = vec![];
    for number in 1..=n {
        if number % 3 == 0 && number % 5 == 0 {
            output.push(String::from("FizzBuzz"));
        } else if number % 3 == 0 {
            output.push(String::from("Fizz"));
        } else if number % 5 == 0 {
            output.push(String::from("Buzz"));
        } else {
            output.push(number.to_string());
        }
    }
    output
}