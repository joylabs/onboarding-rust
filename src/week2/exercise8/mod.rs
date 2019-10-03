use std::collections::HashSet;

pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
    check_row(&board) && check_col(&board) && check_box(&board)
}

fn check_row(board: &Vec<Vec<char>>) -> bool {
    board.iter().all(|row| has_unique_elements(row.to_vec()))
}

fn check_col(board: &Vec<Vec<char>>) -> bool {
    (0..9).all(|col| {
        let all_num = (0..9)
            .fold(&mut Vec::new(), |acc, row| {
                acc.push(board[row][col]);
                acc
            })
            .clone();
        has_unique_elements(all_num)
    })
}

fn check_box(board: &Vec<Vec<char>>) -> bool {
    let mut flag = true;
    flag
}

fn has_unique_elements(vec: Vec<char>) -> bool {
    let mut uniq = HashSet::new();
    vec.into_iter()
        .filter(|c| *c != '.')
        .all(|x| uniq.insert(x))
}
