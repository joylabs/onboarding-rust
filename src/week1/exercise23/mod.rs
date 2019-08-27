pub fn surrounded_regions(mut board: &mut Vec<Vec<char>>) {

    if !board.is_empty() {
        let col_len = board.len();
        let row_len = board[0].len();

        (0..row_len).for_each(|j| {
            if board[0][j] == 'O' {
                search_non_surrounded_region(&mut board, 0, j);
            }
        });

        (0..col_len).for_each(|i| {
            if board[i][0] == 'O' {
                search_non_surrounded_region(&mut board, i, 0);
            }
        });

        (0..col_len).for_each(|i| {
            if board[i][row_len - 1] == 'O' {
                search_non_surrounded_region(&mut board, i, row_len - 1);
            }
        });

        (0..row_len).for_each(|j| {
            if board[col_len - 1][j] == 'O' {
                search_non_surrounded_region(&mut board, col_len - 1, j);
            }
        });

        (0..col_len).for_each(|i| {
            (0..row_len).for_each(|j| {
                if board[i][j] == 'O' {
                    board[i][j] = 'X';
                } else if board[i][j] == 'N' {
                    board[i][j] = 'O';
                }
            })
        });
    }
}

fn search_non_surrounded_region(board: &mut Vec<Vec<char>>, i: usize, j: usize) {

    board[i][j] = 'N';

    if j + 1 < board[0].len() && board[i][j + 1] == 'O' {
        search_non_surrounded_region(board, i, j + 1);
    }
    if i + 1 < board.len() && board[i + 1][j] == 'O' {
        search_non_surrounded_region(board, i + 1, j);
    }

    if j > 0 && board[i][j - 1] == 'O' {
        search_non_surrounded_region(board, i, j - 1);
    }

    if i > 0 && board[i - 1][j] == 'O' {
        search_non_surrounded_region(board, i - 1, j);
    }
}