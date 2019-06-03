pub fn exercise_5(n: i64) -> Vec<String> {
    let mut vector: Vec<String> = vec![];
    for i in 1..=n {
        match i {
            i if i % 3 == 0 && i % 5 == 0 => vector.push("FizzBuzz".to_string()),
            i if i % 3 == 0 => vector.push("Fizz".to_string()),
            i if i % 5 == 0 => vector.push("Buzz".to_string()),
            _ => vector.push(i.to_string()),
        }
    }
    vector
}
