pub fn number_of_islands(mut grid: Vec<Vec<char>>) -> i32 {
    let mut num_islands = 0;
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == '1' {
                num_islands += dfs(&mut grid, i as i32, j as i32);
            }
        }
    }
    num_islands
}

fn dfs(grid: &mut Vec<Vec<char>>, i: i32, j: i32) -> i32 {
    if i < 0
        || j < 0
        || (i as usize) >= grid.len()
        || (j as usize) >= grid[i as usize].len()
        || grid[i as usize][j as usize] == '0'
    {
        return 0;
    }
    grid[i as usize][j as usize] = '0';
    dfs(grid, i + 1, j);
    dfs(grid, i, j + 1);
    dfs(grid, i - 1, j);
    dfs(grid, i, j - 1);
    1
}