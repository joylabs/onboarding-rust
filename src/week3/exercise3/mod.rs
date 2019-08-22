pub fn find_min_arrow_shots(mut points: Vec<Vec<i32>>) -> i32 {

    if points.is_empty() {
        return 0;
    }
    points.sort();
    let mut arrow = points[0][1];

    points.iter().fold(1, |mut acc, ball| {
        if ball[0] <= arrow && arrow >= ball[1] {
            arrow = ball[1];
        } else if ball[0] > arrow {
            acc += 1;
            arrow = ball[1];
        }
        acc
    })
}
