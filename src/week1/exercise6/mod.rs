pub fn plus_one(input: Vec<i32>) -> Vec<i32> {

    let least_index = input.len() - 1;
    let mut result = input.clone();

    result[least_index] += 1;
    for current_place in (1..input.len()).rev() {
        if result[current_place] < 10 {
            return result;
        }

        let more_significant_place = current_place - 1;
        result[current_place] -= 10;
        result[more_significant_place] += 1;
    }

    if result[0] >= 10 {
        result[0] -= 10;
        result.insert(0, 1);
    }

    result
}
