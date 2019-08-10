pub fn surrounded_regions(mut board: &mut Vec<Vec<char>>) {
    // Iterate over board
    for x in 0..board.len() {
        for y in 0..board[x].len() {
            if x == 0
                || x == ((board.len() as i32) - 1) as usize
                || y == 0
                || y == ((board[x].len() as i32) - 1) as usize && board[x][y] == 'O'
            {
                dfs(&mut board, x as i32, y as i32);
            }
        }
    }
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

fn dfs(board: &mut Vec<Vec<char>>, x: i32, y: i32) {
    if board[x as usize][y as usize] == 'O' {
        board[x as usize][y as usize] = '$';
        if x > 0 && board[(x - 1) as usize][y as usize] == 'O' {
            dfs(board, x - 1, y);
        }
        if y < (board[x as usize].len() as i32) - 1 && board[x as usize][(y + 1) as usize] == 'O' {
            dfs(board, x, y + 1);
        }
        if x < (board.len() as i32) - 1 && board[(x + 1) as usize][y as usize] == 'O' {
            dfs(board, x + 1, y);
        }
        if y > 1 && board[x as usize][(y - 1) as usize] == 'O' {
            dfs(board, x, y - 1);
        }
    }
}