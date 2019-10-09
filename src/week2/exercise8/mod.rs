use std::collections::HashSet;

pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
    check_row(&board) && check_col(&board) && check_box(&board)
}

fn check_row(board: &[Vec<char>]) -> bool {
    board.iter().all(|row| has_unique_elements(row.to_vec()))
}

fn check_col(board: &[Vec<char>]) -> bool {
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

fn check_box(board: &[Vec<char>]) -> bool {
    (0..9).all(|i| {
        let sub_box = build_box(board, i, (1 + i / 3) * 3, (i % 3) * 3, (1 + i % 3) * 3);
        has_unique_elements(sub_box)
    })
}

fn build_box(
    board: &[Vec<char>],
    row_start: i32,
    row_end: i32,
    col_start: i32,
    col_end: i32,
) -> Vec<char> {
    let mut sub_box = Vec::<char>::new();
    for (i, j) in iproduct!(row_start..row_end, col_start..col_end) {
        sub_box.push(board[i as usize][j as usize]);
    }
    sub_box
}

fn has_unique_elements(vec: Vec<char>) -> bool {
    let vec = &vec;
    let baseline_lenght = vec.iter().filter(|c| **c != '.').count();

    let uniq = vec
        .iter()
        .filter(|c| **c != '.')
        .fold(&mut HashSet::new(), |acc, c| {
            acc.insert(c);
            acc
        })
        .len();

    baseline_lenght == uniq
}
