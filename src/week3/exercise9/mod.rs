pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
      if intervals.is_empty() {
            return intervals;
      }

      intervals.sort_by(|a, b| a[0].cmp(&b[0]));
      let first_interval = intervals[0].clone();
      let intervals_length = intervals.len() - 1;

      intervals
            .into_iter()
            .enumerate()
            .skip(1)
            .fold(
                  (first_interval, Vec::new()),
                  |(mut current, mut result), (i, interval)| {
                        let merge = is_mergeable(&current, &interval);
                        if merge && (current[1] <= interval[1]) {
                              current = vec![current[0], interval[1]];
                        }

                        if i == intervals_length {
                              if merge {
                                    result.push(current.clone());
                              } else {
                                    result.push(current.clone());
                                    result.push(interval.clone());
                              }
                              (interval, result)
                        } else if merge {
                              (current, result)
                        } else {
                              result.push(current);
                              (interval, result)
                        }
                  },
            )
            .1
}

//Assuming x1 start point is <= x2 start point
fn is_mergeable(x1: &[i32], x2: &[i32]) -> bool {
      x1[1] >= x2[0]
}
