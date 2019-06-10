pub fn matrix_transpose(input: Vec<Vec<i64>>) -> Vec<Vec<i64>> {
    let mut result = vec![];
    let rows = input.len();
    let columns = input[0].len();

    for a in 0..columns {
        let mut new_row = vec![];
        for b in 0..rows {
            new_row.push(input[b][a]);
        }
        result.push(new_row);
    }
    result
}