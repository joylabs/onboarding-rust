pub fn find_circle_num(m: Vec<Vec<i32>>) -> i32 {
    let mut circles = 0;
    let mut visited: Vec<bool> = vec![false; m.len()];
    for i in 0..m.len() {
        if !visited[i as usize] {
            dfs(&m.to_vec(), &mut visited, i as i32);
            circles += 1;
        }
    }
    circles
}

fn dfs(m: &[Vec<i32>], visited: &mut Vec<bool>, current: i32) {
    for j in 0..m[current as usize].len() {
        if m[current as usize][j] == 1 && !visited[j] {
            visited[j] = true;
            dfs(m, visited, j as i32);
        }
    }
}
