use std::cmp;

pub fn min_distance(word1: String, word2: String) -> i32 {
    let len_w1 = word1.len();
    let len_w2 = word2.len();

    if word2.is_empty() {
        return len_w1 as i32;
    } else if word1.is_empty() {
        return len_w2 as i32;
    }

    let mut memoritation: Vec<Vec<usize>> = vec![vec![0; len_w1 + 1]; len_w2 + 1];
    for i in 0..=len_w1 {
        memoritation[0][i] = i;
    }
    for (j, item) in memoritation.iter_mut().enumerate().take(len_w2 + 1) {
        item[0] = j;
    }
    //calculte edit distance
    for (i, w1) in word1.chars().enumerate() {
        for (j, w2) in word2.chars().enumerate() {
            if w1 != w2 {
                memoritation[j + 1][i + 1] = get_min(&memoritation, i, j);
            } else {
                memoritation[j + 1][i + 1] = memoritation[j][i];
            }
        }
    }
    memoritation.pop().unwrap().pop().unwrap() as i32
}

fn get_min(memoritation: &[Vec<usize>], i: usize, j: usize) -> usize {
    let min_up_left = memoritation[j][i];
    let min_up = memoritation[j][i + 1];
    let min_left = memoritation[j + 1][i];
    let min = cmp::min(min_up_left, min_up);
    cmp::min(min, min_left) + 1
}
