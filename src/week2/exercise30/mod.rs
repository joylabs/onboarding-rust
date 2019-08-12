use std::collections::HashSet;

pub fn valid_sudoku(input: Vec<Vec<char>>) -> bool {
	looking_by_box(&input) && rows_are_valid(&input) && columns_are_valid(&input)
}

fn looking_by_box(input: &[Vec<char>]) -> bool {
	let mut boxes = vec![];

	(0..9).step_by(3).for_each(|v| {
		boxes.push(get_box(input, 0, v));
		boxes.push(get_box(input, 3, v));
		boxes.push(get_box(input, 6, v));
	});
	rows_are_valid(&boxes)
}

fn get_box(input: &[Vec<char>], start_col: usize, start_row: usize) -> Vec<char> {

	fn row_portion(row: &[char], start_col: usize) -> Vec<char> {
		row.iter().skip(start_col).take(3).cloned().collect()
	}

	input
		.iter()
		.skip(start_row)
		.map(|row| row_portion(&row, start_col))
		.take(3)
		.flatten()
		.collect()
}

fn rows_are_valid(input: &[Vec<char>]) -> bool {
	let mut seen: HashSet<char> = HashSet::new();

	input.iter().all(|x| {
		seen.clear();
		x.iter().all(|y| verify_existing_numbers(*y, &mut seen))
	})
}

fn columns_are_valid(input: &[Vec<char>]) -> bool {
	let mut seen: HashSet<char> = HashSet::new();
	(0..9).all(|x| {
		seen.clear();
		input
			.iter()
			.all(|y| verify_existing_numbers(y[x], &mut seen))
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
