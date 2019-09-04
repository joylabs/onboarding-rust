pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
    let mut num = 0;
    let mut cloned_grid = grid.clone();
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if cloned_grid[i][j] == '1' {
                num += 1;
                island(&mut cloned_grid, i as i32, j as i32);
            }
        }
    }
    num
}

fn island(grid: &mut Vec<Vec<char>>, i: i32, j: i32) {
    if i >= 0
        && i < grid.len() as i32
        && j >= 0
        && j < grid[i as usize].len() as i32
        && grid[i as usize][j as usize] == '1'
    {
        grid[i as usize][j as usize] = '0';
        island(grid, i, j + 1);
        island(grid, i, j - 1);
        island(grid, i + 1, j);
        island(grid, i - 1, j);
    }
}
