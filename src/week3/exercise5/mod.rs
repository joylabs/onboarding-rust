pub fn partition_labels(s: String) -> Vec<i32> {
      let mut result: Vec<i32> = Vec::new();
      let mut letter_indexes: Vec<Vec<i32>> = Vec::new();
      let mut letters: Vec<char> = Vec::new();
      
      s.chars().enumerate().for_each(|(i, c)|{
            if !letters.contains(&c) {
                  let last_index = find_last_index(c, &s) as i32;
                  letters.push(c);
                  letter_indexes.push(vec![i as i32, last_index]);
            }
      });

      let mut group_indexes = letter_indexes[0].clone();
      letter_indexes.iter().skip(1).for_each(|indexes| {
            if is_intersection(&group_indexes, &indexes) {
                  if group_indexes[1] < indexes[1] {
                        group_indexes[1] = indexes[1];
                  }
            } else {
                  let distance = group_indexes[1] - group_indexes[0] + 1;
                  result.push(distance);
                  group_indexes = indexes.clone();
            }
      });
      let distance = group_indexes[1] - group_indexes[0] + 1;
      result.push(distance);
      result
}

fn find_last_index(letter: char, word: &str) -> usize {
      word.chars().enumerate().fold(0, |acc, (i, c)| {
            if letter == c {
                  i
            } else {
                  acc
            }
      })
}

//Assuming x1 start point is <= x2 start point
fn is_intersection(x1: &[i32], x2: &[i32]) -> bool {
      x1[1] >= x2[0]
}