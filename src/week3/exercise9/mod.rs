pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
      if intervals.is_empty() {
            return intervals;
      }

      intervals.sort();
      let mut interval = intervals[0].clone();
      let mut result: Vec<Vec<i32>> = Vec::new();
      let mut i = 1;

      while i < intervals.len() {
            if is_mergeable(&interval, &intervals[i]) {
                  if interval[1] <= intervals[i][1] {
                        interval = vec![interval[0], intervals[i][1]];
                  } else {
                        interval = vec![interval[0], interval[1]];
                  }
            } else {
                  result.push(interval.clone());
                  interval = intervals[i].clone();
            }
            i += 1;
      }

      result.push(interval);

      result
}

//Assuming x1 start point is <= x2 start point
fn is_mergeable(x1: &[i32], x2: &[i32]) -> bool {
      x1[1] >= x2[0]
}
