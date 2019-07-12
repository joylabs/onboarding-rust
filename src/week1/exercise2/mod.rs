pub fn transpose_matrix(v: Vec<Vec<i64>>) -> Vec<Vec<i64>> {
    let mut result = vec![];
    for (ix, _x) in v[0].iter().enumerate() {
        let mut new_row: Vec<i64> = vec![];
        for (iy, _) in v.iter().enumerate() {
            new_row.push(v[iy][ix]);
        }
        result.push(new_row);
    }

    result
}
