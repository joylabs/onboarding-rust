pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
      intervals.sort_by(|a, b| a[0].cmp(&b[0]));

      intervals
}

//Assuming x1 start point is <= x2 start point
fn is_intersection(x1: &[i32], x2: &[i32]) -> bool {
      x1[1] >= x2[0]
}