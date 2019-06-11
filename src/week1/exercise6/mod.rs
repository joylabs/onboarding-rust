pub fn plus_one(input: Vec<i32>) -> Vec<i32> {

    let least_index = input.len() - 1;
    let mut result = input.clone();
    let mut a = 0;

    result[least_index] += 1;
    while a <= least_index && result[least_index - a] >= 10 {
        result[least_index - a] -= 10;
        a += 1;
        if a > least_index {
            result.insert(0, 1);
        } else {
            result[least_index - (a)] += 1;
        }
    }
    result
}