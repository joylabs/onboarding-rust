pub fn sorrounded_regions(regions: &mut Vec<Vec<char>>) {
    let mut board = regions.clone();

    (0..board.len()).for_each(|i| {
        (0..board[0].len()).for_each(|j| {
            if board[i][j] == 'O' {
                if recursiva_searching_border(&mut board, i, j) {
                    fill_with(&mut board, i, j, '0');
                } else {
                    fill_with(regions, i, j, 'X');
                }
            }
        })
    });
}

fn is_border(i: usize, j: usize, x: usize, y: usize) -> bool {

    if i == 0 {
        return true;
    }
    if i == x - 1 {
        return true;
    }
    if j == 0 {
        return true;
    }
    if j == y - 1 {
        return true;
    }
    false
}

fn recursiva_searching_border(border: &mut Vec<Vec<char>>, i: usize, j: usize) -> bool {

    border[i][j] = 'X';
    if is_border(i, j, border.len(), border[0].len()) {
        border[i][j] = 'O';
        return true;
    }

    if j + 1 < border[0].len()
        && border[i][j + 1] == 'O'
        && recursiva_searching_border(border, i, j + 1)
    {
        border[i][j] = 'O';
        return true;
    }

    if i + 1 < border.len()
        && border[i + 1][j] == 'O'
        && recursiva_searching_border(border, i + 1, j)
    {
        border[i][j] = 'O';
        return true;
    }

    if j > 0 && border[i][j - 1] == 'O' && recursiva_searching_border(border, i, j - 1) {
        border[i][j] = 'O';
        return true;
    }

    if i > 0 && border[i - 1][j] == 'O' && recursiva_searching_border(border, i - 1, j) {
        border[i][j] = 'O';
        return true;
    }

    false
}

fn fill_with(lands: &mut Vec<Vec<char>>, i: usize, j: usize, ch: char) {
    lands[i][j] = ch;
    if j + 1 < lands[0].len() && lands[i][j + 1] == 'O' {
        fill_with(lands, i, j + 1, ch);
    }
    if i + 1 < lands.len() && lands[i + 1][j] == 'O' {
        fill_with(lands, i + 1, j, ch);
    }
    if j > 0 && lands[i][j - 1] == 'O' {
        fill_with(lands, i, j - 1, ch);
    }
    if i > 0 && lands[i - 1][j] == 'O' {
        fill_with(lands, i - 1, j, ch);
    }
}