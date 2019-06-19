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

// OPTION TWO
// let least_index = input.len() as i32 - 1;
// let mut current_index = least_index;
// let mut result = input.clone();

// result[current_index as usize] += 1;
// while current_index >= 0 && result[current_index as usize] >= 10 {
//     result[current_index as usize] -= 10;
//     current_index -= 1;
//     if current_index < 0 {
//         result.insert(0, 1);
//     } else {
//         result[current_index as usize] += 1;
//     }
// }

// result

// OPTION ONE
// let least_index = input.len() - 1;
// let mut result = input.clone();
// let mut a = 0;

// result[least_index] += 1;
// while a <= least_index && result[least_index - a] >= 10 {
//     result[least_index - a] -= 10;
//     a += 1;
//     if a > least_index {
//         result.insert(0, 1);
//     } else {
//         result[least_index - (a)] += 1;
//     }
// }
// result
