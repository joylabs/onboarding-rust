pub fn find_min_arrow_shots(mut points: Vec<Vec<i32>>) -> i32 {
      if points.is_empty() {
            return 0;
      }

      points.sort_by(|a, b| a[0].cmp(&b[0]));
      let mut intersection_groups = Vec::new();

      points.into_iter().for_each(|balloon| {
            insert_into_groups(balloon, &mut intersection_groups);
      });

      intersection_groups.len() as i32
}

//Assuming x1 start point is <= x2 start point
fn is_intersection(x1: &[i32], x2: &[i32]) -> bool {
      x1[1] >= x2[0]
}

fn insert_into_groups(balloon: Vec<i32>, intersection_groups: &mut Vec<Vec<Vec<i32>>>) {
      if intersection_groups.is_empty() {
            intersection_groups.push(vec![balloon]);
      } else {
            let mut belongs_to_group = true;

            for group in intersection_groups.iter_mut() {
                  for ballon_element in group.iter() {
                        if !is_intersection(ballon_element, &balloon) {
                              belongs_to_group = false;
                        }
                  }
                  if belongs_to_group {
                        group.push(balloon);
                        return;
                  } else {
                        belongs_to_group = true;
                  }
            }

            intersection_groups.push(vec![balloon]);
      }
}