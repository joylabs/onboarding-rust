use std::collections::HashSet;

pub fn valid_sudoku(input: Vec<Vec<char>>) -> bool {
    looking_into_rows(input.clone()) && looking_into_col(input.clone()) && looking_in_box(input)
}

fn looking_in_box(input: Vec<Vec<char>>) -> bool {
    let vectors = input.clone();
    let mut vec1: Vec<char> = vec![];
    let mut vec2: Vec<char> = vec![];
    let mut vec3: Vec<char> = vec![];

    for row in vectors {
        for (j, el) in row.iter().enumerate() {
            if j < 3 {
                vec1.push(*el);
            } else if j < 6 {
                vec2.push(*el);
            } else {
                vec3.push(*el);
            }
        }
    }
    box_by_box(vec1) && box_by_box(vec2) && box_by_box(vec3)
}

fn box_by_box(vec: Vec<char>) -> bool {
    let mut boxes: HashSet<char> = HashSet::new();

    for (i, ch) in vec.iter().enumerate() {
        if *ch != '.' {
            if boxes.contains(ch) {
                return false;
            } else {
                boxes.insert(*ch);
            }
        }
        if i == 8 || i == 17 {
            boxes.clear();
        }
    }
    true
}


fn looking_into_rows(input: Vec<Vec<char>>) -> bool {
    let mut row: HashSet<char> = HashSet::new();
    input.into_iter().all(|mut x| {
        row.clear();
        x.sort_by(|a, b| a.cmp(b).reverse());
        println!("{:?}", x);
        x.iter().all(|y| {
            if *y != '.' {
                if row.contains(y) {
                    return false;
                } else {
                    row.insert(*y);
                }
            } else {
                return true;
            }
            true
        })
    })
}

fn looking_into_col(input: Vec<Vec<char>>) -> bool {
    let mut col: HashSet<char> = HashSet::new();

    (0..9).all(|x| {
        col.clear();
        input.iter().all(|y| {
            if y[x] != '.' {
                if col.contains(&y[x]) {
                    return false;
                } else {
                    col.insert(y[x]);
                }
            }
            true
        })
    })
}