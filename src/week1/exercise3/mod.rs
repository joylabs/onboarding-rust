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
