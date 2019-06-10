pub fn matrix_flip_inverse(input: Vec<Vec<i64>>) -> Vec<Vec<i64>> {
    let mut result = vec![];
    let columns = input[0].len();

    for (a, _) in input.iter().enumerate() {
        let mut new_row = vec![];
        for b in (0..columns).rev() {
            new_row.push(1 - input[a][b]);
        }
        result.push(new_row);
    }
    result
}

// ONE_LINE
// impl Solution {
//     pub fn flip_and_invert_image(a: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
//         a.into_iter().map(|inner_vec| inner_vec.into_iter().rev().map(|n| 1-n).collect()).collect()
//     }
// }