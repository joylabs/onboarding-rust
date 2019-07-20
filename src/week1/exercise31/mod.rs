use std::collections::HashSet;

pub fn is_valid_sudoku(input: Vec<Vec<char>>) -> bool {
    for i in 0..9 {
        let mut column = Vec::new();

        for row in input.iter() {
            if !is_valid_line(row) {
                return false;
            }
            column.push(row[i])
        }

        if !is_valid_line(&column) {
            return false;
        }
    }

    eval_sub_boxes(&input)
}

fn is_valid_line(input: &[char]) -> bool {
    let total_elements = input.iter().fold(0, |mut acc, x| {
        if *x != '.' {
            acc += 1;
        }
        acc
    });

    let unique_elements: HashSet<char> = input
        .iter()
        .filter_map(|x| if *x != '.' { Some(*x) } else { None })
        .collect();

    total_elements == unique_elements.len()
}

fn eval_sub_boxes(input: &[Vec<char>]) -> bool {
    for j in (0..9).step_by(3) {
        let mut sub_box: Vec<char> = Vec::new();
        for i in (0..9).step_by(3) {
            let k = j + 3;

            sub_box.extend_from_slice(&input[i][j..k]);
            sub_box.extend_from_slice(&input[i + 1][j..k]);
            sub_box.extend_from_slice(&input[i + 2][j..k]);

            if !is_valid_line(&sub_box) {
                return false;
            }

            sub_box = Vec::new();
        }
    }
    true
}