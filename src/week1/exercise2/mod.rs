pub fn transpose_matrix(input: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut output = vec![];
    for row_element in 0..input[0].len() {
        let mut row = vec![];
        for col_element in 0..input.len() {
            row.push(input[col_element][row_element]);
        }
        output.push(row);
    }
    output
}