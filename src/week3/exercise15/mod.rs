pub fn is_match(s: &str, p: &str) -> bool {
	let mut matrix: Vec<Vec<bool>> = vec![vec![false; p.len() + 1]; s.len() + 1];
	matrix[0][0] = true;

	p.chars()
		.enumerate()
		.filter(|(_, ch)| *ch == '*')
		.for_each(|(i, _)| matrix[0][i + 1] = matrix[0][i - 1]);

	s.chars().enumerate().for_each(|(i, a)| {
		build_matrix(&mut matrix, p, a, i);
	});
	matrix.pop().unwrap().pop().unwrap()
}

fn build_matrix(map: &mut [Vec<bool>], p: &str, a: char, i: usize) {
	let mut befor: char = '\0';
	p.chars().enumerate().for_each(|(j, b)| {
		if !is_matching(a, b, map, befor, i, j) {
			map[i + 1][j + 1] = false;
		}
		befor = b;
	})
}

fn is_matching(
	a: char,
	b: char,
	matrix: &mut [Vec<bool>],
	befor: char,
	i: usize,
	j: usize,
) -> bool {
	if a == b || b == '.' {
		matrix[i + 1][j + 1] = matrix[i][j];
		return true;
	} else if b == '*' {
		matrix[i + 1][j + 1] = matrix[i + 1][j - 1];
		if befor == '.' || befor == a {
			matrix[i + 1][j + 1] = matrix[i][j + 1] | matrix[i + 1][j - 1];
		}
		return true;
	}
	false
}
