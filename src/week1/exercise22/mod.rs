pub fn number_of_islands(grid: Vec<Vec<char>>) -> i32 {

    let mut lands = grid.clone();

    (0..lands.len()).fold(0, |islas, i| {
        islas
            + (0..lands[0].len()).fold(0, |isla, j| {
                if lands[i][j] == '1' {
                    searching_land(&mut lands, i, j);
                    isla + 1
                } else {
                    isla
                }
            })
    })

}


fn searching_land(lands: &mut Vec<Vec<char>>, i: usize, j: usize) {

    lands[i][j] = '0';

    if j + 1 < lands[0].len() && lands[i][j + 1] == '1' {
        searching_land(lands, i, j + 1);
    }
    if i + 1 < lands.len() && lands[i + 1][j] == '1' {
        searching_land(lands, i + 1, j);
    }

    if j > 0 && lands[i][j - 1] == '1' {
        searching_land(lands, i, j - 1);
    }

    if i > 0 && lands[i - 1][j] == '1' {
        searching_land(lands, i - 1, j);
    }
}