pub fn transpose_matrix(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    // Vec::new()
    let mut output = Vec::new();
    let column = matrix[0].len();
    // Input: [ [1,2,3],[4,5,6],[7,8,9] ]
    for y_index in 0..column {
        let mut row = Vec::new();
        for (x_index, _) in matrix.iter().enumerate() {
            row.push(matrix[x_index][y_index]);
        }
        output.push(row);
    }
    output
}
