use std::collections::HashMap;

pub fn valid_sudoku(input: Vec<Vec<&str>>) -> bool {
    let mut repeat: HashMap<&str, i32> = HashMap::new();

    input.iter().any(|x| {
        x.iter().any(|y| {
            if *y != "." {
                if repeat.contains_key(y) {
                    print!("if{}", y);
                    return true;
                } else {
                    print!("else{}", y);
                    repeat.entry(*y).or_insert(1);
                }
            }
            false
        })
    });
    print!("{:?}", repeat);
    false
}
