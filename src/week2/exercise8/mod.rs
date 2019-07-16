use std::collections::HashSet;

pub fn valid_sudoku(input: Vec<Vec<char>>) -> bool {
    let mut seen: HashSet<char> = HashSet::new();
    rows_are_valid(&input, &mut seen) && columns_are_valid(&input, &mut seen) && looking_in_box(input, &mut seen)
}

fn looking_in_box(input: Vec<Vec<char>>, seen: &mut HashSet<char>) -> bool {
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
    box_by_box(vec1, seen) && box_by_box(vec2, seen) && box_by_box(vec3, seen)
}

fn box_by_box(vec: Vec<char>, seen: &mut HashSet<char>) -> bool {
    seen.clear();

    for (i, ch) in vec.iter().enumerate() {
        verify_existing_numbers(*ch, seen);
        if i == 8 || i == 17 {
            seen.clear();
        }
    }
    true
}

fn rows_are_valid(input: &Vec<Vec<char>>, seen: &mut HashSet<char>) -> bool {
    seen.clear();

    input.iter().all(|x| {
        seen.clear();
        x.iter().all(|y| verify_existing_numbers(*y, seen))
    })
}

fn columns_are_valid(input: &Vec<Vec<char>>, seen: &mut HashSet<char>) -> bool {
    seen.clear();

    (0..9).all(|x| {
        seen.clear();
        input
            .iter()
            .all(|y| verify_existing_numbers(y[x], seen))
    })
}

fn verify_existing_numbers(input: char, hash: &mut HashSet<char>) -> bool {
    if input != '.' {
        if hash.contains(&input) {
            return false;
        } else {
            hash.insert(input);
        }
    }
    true
}