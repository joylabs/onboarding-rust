pub fn capture_surrounded_regions(board: &mut Vec<Vec<char>>) {
    iterate_over_board(board);
    change_asterisks_in_board(board);
}

fn iterate_over_board(board: &mut Vec<Vec<char>>) {
    for i in 0..board.len() {
        for j in 0..board[i].len() {
            if i == 0 || i == board.len() - 1 || j == 0 || j == board[0].len() - 1 {
                find_regions(board, i as i32, j as i32);
            }
        }
    }
}

fn find_regions(board: &mut Vec<Vec<char>>, i: i32, j: i32) {
    let col_len = board[0].len() as i32;
    if is_in_bounds(i, j, board.len() as i32, col_len) && board[i as usize][j as usize] == 'O' {
        board[i as usize][j as usize] = '*';
        find_regions(board, i, j + 1);
        find_regions(board, i, j - 1);
        find_regions(board, i + 1, j);
        find_regions(board, i - 1, j);
    }
}

fn is_in_bounds(row: i32, column: i32, row_len: i32, col_len: i32) -> bool {
    println!("i = {} - j = {}", row, column);
    row >= 0 && row < row_len && column >= 0 && column < col_len
}

fn change_asterisks_in_board(board: &mut Vec<Vec<char>>) {
    board.iter_mut().for_each(|row| {
        row.iter_mut().for_each(|region| match region {
            '*' => *region = 'O',
            _ => *region = 'X',
        });
    });
}
