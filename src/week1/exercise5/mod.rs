pub fn fizz_buzz(input: i32) -> Vec<String> {
    let mut output = vec![];
    for i in 1..=input {
        if i % 3 == 0 && i % 5 == 0 {
            output.push(String::from("FizzBuzz"));
        } else if i % 3 == 0 {
            output.push(String::from("Fizz"));
        } else if i % 5 == 0 {
            output.push(String::from("Buzz"));
        } else {
            output.push(i.to_string());
        }
    }
    output
}