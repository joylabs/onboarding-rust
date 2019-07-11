pub fn flip_and_invert_image(a: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    a.into_iter()
        .map(|mut r| {
            r.reverse();
            r
        })
        .map(|v| v.iter().map(|v| v ^ 1).collect())
        .collect::<Vec<Vec<i32>>>()
}
