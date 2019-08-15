
pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
    let mut board_2 = board.clone();

    board.iter().enumerate().any(|(i, vec)| {

        let x = vec
            .iter()
            .enumerate()
            .find(|(j, _)| recursiva_searching_word(&mut board_2, i, *j, word.clone()));
        x.is_some()
    })
}

fn recursiva_searching_word(
    board: &mut Vec<Vec<char>>,
    i: usize,
    j: usize,
    mut word: String,
) -> bool {
    if word.is_empty() {
        return true;
    }
    if i == board.len() || j == board[0].len() {
        return false;
    }

    let ch = word.remove(0);

    if ch != board[i][j] {
        return false;
    }

    let cur = board[i][j];
    board[i][j] = '0';

    if recursiva_searching_word(board, i, j + 1, word.clone()) {
        return true;
    }

    if recursiva_searching_word(board, i + 1, j, word.clone()) {
        return true;
    }

    if j > 0 && recursiva_searching_word(board, i, j - 1, word.clone()) {
        return true;
    }

    if i > 0 && recursiva_searching_word(board, i - 1, j, word.clone()) {
        return true;
    }

    board[i][j] = cur;
    false
}
