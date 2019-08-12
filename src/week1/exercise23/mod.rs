pub fn surrounded_regions(mut board: &mut Vec<Vec<char>>) {
    let row = board.len();
    if row == 0 {
        return;
    }
    let col = board[0].len();


    // Iterate over board
    (0..row).for_each(|x| {
        (0..col).for_each(|y| {
            if [0, row - 1].contains(&x) || [0, col - 1].contains(&y) && board[x][y] == 'O' {
                dfs(&mut board, x, y);
            }
        });
    });
    for x in board {
        for y in x.iter_mut() {
            if *y == 'O' {
                *y = 'X';
            }
            if *y == '$' {
                *y = 'O';
            }
        }
    }
}

fn dfs(board: &mut Vec<Vec<char>>, x: usize, y: usize) {
    if (0..board.len()).contains(&x) && (0..board[0].len()).contains(&y) && board[x][y] == 'O' {
        board[x][y] = '$';
        if let Some(xi) = x.checked_sub(1) {
            dfs(board, xi, y);
        }
        dfs(board, x, y + 1);
        dfs(board, x + 1, y);
        if let Some(yi) = y.checked_sub(1) {
            dfs(board, x, yi);
        }
    }
}
