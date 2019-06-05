pub fn matrix_flip_inverse(input: Vec<Vec<i64>>) -> Vec<Vec<i64>> {
    
    let mut result = vec![];
    let rows = input.len();
    let columns = input[0].len();

    for a in 0..rows {
        let mut newRow = vec![];
        for b in (0..columns).rev(){
            newRow.push(1-input[a][b]);
        }
        result.push(newRow);
    }
    result
}