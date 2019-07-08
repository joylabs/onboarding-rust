pub fn sorrounded_regions(board: &mut Vec<Vec<char>>) {

    if board.is_empty() {
        return;
    }

    // looking 'O' in border by rows
    (0..board[0].len()).for_each(|i| {
        if board[0][i] == 'O' {
            recursive_fill(board, 0, i, '0');
        }
        if board[board.len() - 1][i] == 'O' {
            recursive_fill(board, board.len() - 1, i, '0');
        }
    });

    // looking 'O' in border by col
    (0..board.len()).for_each(|j| {
        if board[j][0] == 'O' {
            recursive_fill(board, j, 0, '0');
        }
        if board[j][board[0].len() - 1] == 'O' {
            recursive_fill(board, j, board[0].len() - 1, '0');
        }
    });

    //replacing all 'O' no-in-border with 'X' and all the '0' by 'O'
    board.iter_mut().for_each(|row| {
        row.iter_mut().for_each(|ch| {
            if *ch == 'O' {
                *ch = 'X'
            } else if *ch == '0' {
                *ch = 'O'
            }
        })
    });
}

fn recursive_fill(regions: &mut Vec<Vec<char>>, i: usize, j: usize, ch: char) {

    regions[i][j] = ch;
    // right side searching
    if j + 1 < regions[0].len() && regions[i][j + 1] == 'O' {
        recursive_fill(regions, i, j + 1, ch);
    }
    // down searching
    if i + 1 < regions.len() && regions[i + 1][j] == 'O' {
        recursive_fill(regions, i + 1, j, ch);
    }
    // left side searching
    if j > 0 && regions[i][j - 1] == 'O' {
        recursive_fill(regions, i, j - 1, ch);
    }
    // over searching
    if i > 0 && regions[i - 1][j] == 'O' {
        recursive_fill(regions, i - 1, j, ch);
    }
}