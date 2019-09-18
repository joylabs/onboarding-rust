pub fn partition_labels(s: String) -> Vec<i32> {
    let letter_indexes = get_start_end_letter_indexes(&s);
    let letter_indexes_len = letter_indexes.len() - 1;

    if letter_indexes_len == 0 {
        return vec![s.len() as i32];
    }

    letter_indexes
        .into_iter()
        .enumerate()
        .fold(
            (Vec::new(), Vec::new()),
            |(mut group_indexes, mut result), (i, indexes)| {
                if i == 0 {
                    (indexes, Vec::new())
                } else {
                    let intersection = is_intersection(&group_indexes, &indexes);
                    if intersection && group_indexes[1] < indexes[1] {
                        group_indexes[1] = indexes[1];
                    }
                    if i == letter_indexes_len {
                        let distance = group_indexes[1] - group_indexes[0] + 1;
                        result.push(distance);
                        if !intersection {
                            let distance = indexes[1] - indexes[0] + 1;
                            result.push(distance);
                        }
                        (indexes, result)
                    } else if intersection {
                        (group_indexes, result)
                    } else {
                        let distance = group_indexes[1] - group_indexes[0] + 1;
                        result.push(distance);
                        (indexes, result)
                    }
                }
            },
        )
        .1
}

fn get_start_end_letter_indexes(word: &str) -> Vec<Vec<i32>> {
    word.chars()
        .enumerate()
        .fold(
            (Vec::new(), Vec::new()),
            |(mut letters, mut group_indexes), (i, c)| {
                if !letters.contains(&c) {
                    let last_index = find_last_index(c, word);
                    group_indexes.push(vec![i as i32, last_index]);
                    letters.push(c);
                }
                (letters, group_indexes)
            },
        )
        .1
}

fn find_last_index(letter: char, word: &str) -> i32 {
    word.chars()
        .enumerate()
        .fold(0, |acc, (i, c)| if letter == c { i as i32 } else { acc })
}

//Assuming x1 start point is <= x2 start point
fn is_intersection(x1: &[i32], x2: &[i32]) -> bool {
    x1[1] >= x2[0]
}