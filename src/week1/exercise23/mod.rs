pub fn capture_surrounded_regions(board: &mut Vec<Vec<char>>) {

    for i in 0..board.len() {
        for j in 0..board[i].len() {
            if board[i][j] == 'O'
                && (i == 0 || i == board.len() - 1 || j == 0 || j == board[0].len() - 1)
            {
                find_regions(board, i as i32, j as i32);
            }
        }
    }

    board.iter_mut().for_each(|row| {
        row.iter_mut().for_each(|region| match region {
            '*' => *region = 'O',
            _ => *region = 'X',
        });
    });
}

fn find_regions(board: &mut Vec<Vec<char>>, i: i32, j: i32) {
    if i >= 0
        && i < board.len() as i32
        && j >= 0
        && j < board[i as usize].len() as i32
        && board[i as usize][j as usize] == 'O'
    {
        board[i as usize][j as usize] = '*';
        find_regions(board, i, j + 1);
        find_regions(board, i, j - 1);
        find_regions(board, i + 1, j);
        find_regions(board, i - 1, j);
    }

}
