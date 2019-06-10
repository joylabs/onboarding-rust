pub fn matrix_transpose(input: Vec<Vec<i64>>) -> Vec<Vec<i64>> {
    let mut result = vec![];
    let columns = input[0].len();

    for col in 0..columns {
        let mut new_row = vec![];
        for (row, _) in input.iter().enumerate() {
            new_row.push(input[row][col]);
        }
        result.push(new_row);
    }
    result
}

// ONE_LINE
// pub fn matrix_transpose(input: Vec<Vec<i64>>) -> Vec<Vec<i64>> {
//     let width = input[0].len();
//     let iter = input.into_iter().flatten();
//     (0..width)
//         .map(|i| iter.clone().skip(i).step_by(width).collect())
//         .collect()
// }