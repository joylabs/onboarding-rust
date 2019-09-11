pub fn find_words(board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
    if board.is_empty() {
        return vec![];
    }

    words
        .into_iter()
        .filter(|w| find_word(&board, w.chars().collect()))
        .collect()
}

fn find_word(board: &[Vec<char>], word: Vec<char>) -> bool {
    let mut visited_in_board = vec![vec![false; board[0].len()]; board.len()];

    for row in 0..board.len() {
        for col in 0..board[0].len() {
            if search_for_word(
                board,
                &mut visited_in_board,
                &word,
                0,
                row as i32,
                col as i32,
            ) {
                return true;
            }
        }
    }

    false
}

fn search_for_word(
    board: &[Vec<char>],
    visited_in_board: &mut Vec<Vec<bool>>,
    word: &[char],
    position: usize,
    row: i32,
    col: i32,
) -> bool {
    if is_out_of_bounds(row, col, board.len() as i32, board[0].len() as i32)
        || have_visited(&visited_in_board, row, col)
        || board[row as usize][col as usize] != word[position]
    {
        return false;
    }

    let new_position = position + 1;

    if new_position == word.len() {
        return true;
    }

    visited_in_board[row as usize][col as usize] = true;

    let found = search_for_word(board, visited_in_board, &word, new_position, row + 1, col)
        || search_for_word(board, visited_in_board, &word, new_position, row - 1, col)
        || search_for_word(board, visited_in_board, &word, new_position, row, col + 1)
        || search_for_word(board, visited_in_board, &word, new_position, row, col - 1);

    visited_in_board[row as usize][col as usize] = false;

    found
}

fn is_out_of_bounds(row: i32, col: i32, board_heigth: i32, board_width: i32) -> bool {
    row < 0 || row >= board_heigth || col < 0 || col >= board_width
}

fn have_visited(visited_in_board: &[Vec<bool>], row: i32, col: i32) -> bool {
    visited_in_board[row as usize][col as usize]
}
