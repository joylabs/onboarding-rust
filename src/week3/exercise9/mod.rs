pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    if intervals.is_empty() {
        return intervals;
    }

    intervals.sort();

    let mut max = intervals[0][1];
    let mut min = intervals[0][0];
    let mut merge: Vec<Vec<i32>> = vec![];

    for (i, vec) in intervals.iter().enumerate() {
        if vec[0] <= max && vec[1] > max {
            max = vec[1];
        }
        if vec[0] > max {
            merge.push(vec![min, max]);
            min = vec[0];
            max = vec[1];
        }
        if intervals.len() - 1 == i {
            merge.push(vec![min, max]);
        }
    }
    merge
}