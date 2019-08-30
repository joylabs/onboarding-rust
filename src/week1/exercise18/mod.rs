pub fn single_number(input: Vec<i32>) -> i32 {
    input.iter().fold(0, |single, num| single ^ num)
}


