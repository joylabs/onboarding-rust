pub fn exercise_5(n: i64) -> Vec<String> {
    let mut vector: Vec<String> = vec![];
    for i in 1..=n {
        if i % 3 == 0 && i % 5 == 0 {
            // println!("FizzBuzz");
            vector.push("FizzBuzz".to_string());
        } else if i % 3 == 0 {
            // println!("Fizz");
            vector.push("Fizz".to_string());
        } else if i % 5 == 0 {
            //println!("Buzz");
            vector.push("Buzz".to_string());
        } else {
            //println!("{}", i);
            vector.push(i.to_string());
        }
    }
    vector
}
