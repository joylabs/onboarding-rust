pub fn plus_one(input: Vec<i32>) -> Vec<i32> {
    let mut output: Vec<i32> = input.clone();
    for number in output.iter_mut().rev() {
        if *number < 9 {
            *number += 1;
            return output;
        } else {
            *number = 0;
        }
    }
    output.insert(0, 1);
    output
}