pub fn find_min_arrow_shots(mut points: Vec<Vec<i32>>) -> i32 {
    if points.is_empty() {
        return 0;
    }
    points.sort();
    let mut arrow = points[0][1];

    points.iter().skip(1).fold(1, |mut acc, balloon| {
        if arrow >= balloon[1] {
            arrow = balloon[1];
        } else if balloon[0] > arrow {
            acc += 1;
            arrow = balloon[1];
        }
        acc
    })
}
