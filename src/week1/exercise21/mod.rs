pub fn find_circle_num(input: Vec<Vec<i32>>) -> i32 {
    let n = input.len();
    let mut circles = 0;
    let mut visited = vec![0; n];
    (0..n).for_each(|i| {
        if visited[i] == 0 {
            dfs(i, &input, &mut visited);
            circles += 1;
        }
    });
    circles
}

fn dfs(i: usize, m: &[Vec<i32>], visited: &mut Vec<i32>) {
    visited[i] = 1;
    (0..m.len()).for_each(|j| {
        if m[i][j] == 1 && visited[j] == 0 {
            dfs(j, m, visited);
        }
    });
}

pub fn find_circle_num_2(input: Vec<Vec<i32>>) -> i32 {
    let mut friends_relation = input.clone();

    (0..friends_relation.len()).fold(0, |circles, i| {
        circles
            + (0..friends_relation[0].len()).fold(0, |circle, j| {
                if friends_relation[i][j] == 1 {
                    searching_circle(&mut friends_relation, i, j);
                    circle + 1
                } else {
                    circle
                }
            })
    })
}
fn searching_circle(friends_relation: &mut Vec<Vec<i32>>, i: usize, j: usize) {

    friends_relation[i][j] = 0;

    (0..friends_relation[0].len()).for_each(|j2| {
        if friends_relation[j][j2] == 1 {
            searching_circle(friends_relation, j, j2);
        }
    });

    (0..friends_relation[0].len()).for_each(|j2| {
        if friends_relation[i][j2] == 1 {
            searching_circle(friends_relation, i, j2);
        }
    });
}
