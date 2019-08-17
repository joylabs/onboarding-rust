
pub fn find_content_children(mut g: Vec<i32>, mut s: Vec<i32>) -> i32 {
    g.sort();
    s.sort();

    (0..s.len()).fold(0, |mut acc, x| {
        if acc < g.len() as i32 && s[x] >= g[acc as usize] {
            acc += 1;
        }
        acc
    })
}

//let Some(index) = s.iter().position(|cooki| cooki >= child)
