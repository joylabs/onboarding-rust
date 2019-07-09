
pub fn single_number(input: Vec<i32>) -> i32 {
    /*
        FIRST ATTEMPT:
        Using a HashMap and finding the Key with Value == 1
        Works but it's very slow

        // Create new HashMap for input
        let mut input_numbers = HashMap::new();
        // Load input into a HashMap
        for n in input.iter() {
            let count = input_numbers.entry(n).or_insert(0);
            *count += 1;
        }
        println!("{:?}", input_numbers);

        // Use find() function to return the first element found with value = 1
        *input_numbers.into_iter().find(|(_, x)| *x == 1).unwrap().0
    */

    // SECOND ATTEMPT
    // Runs faster and it's more readable
    let mut a = 0;
    for n in input.iter() {
        a ^= n;
    }
    a
}