use std::collections::HashMap;

pub fn partition_labels(s: String) -> Vec<i32> {
    //find all the position for each char
    let s_positions = s.chars().enumerate().fold(
        HashMap::new(),
        |mut acc: HashMap<char, Vec<usize>>, (i, ch)| {
            let v = acc.entry(ch).or_insert_with(|| vec![]);
            v.push(i);
            acc
        },
    );
    // get the first and the last position of each char
    let mut vec: Vec<Vec<usize>> = s_positions
        .iter()
        .map(|(_, vec)| vec![vec[0], vec[vec.len() - 1]])
        .collect();
    vec.sort();
    count_characters(vec)
}

fn count_characters(points: Vec<Vec<usize>>) -> Vec<i32> {
    let mut arrow = points[0][1];
    let mut num_str: i32 = -1;
    points
        .iter()
        .enumerate()
        .fold(vec![], |mut acc, (i, ball)| {
            if ball[0] <= arrow && arrow < ball[1] {
                arrow = ball[1];
            } else if ball[0] > arrow {
                acc.push(arrow as i32 - num_str);
                num_str = arrow as i32;
                arrow = ball[1];
            }
            if points.len() - 1 == i {
                acc.push(arrow as i32 - num_str);
            }
            acc
        })
}
//"ababcbacadefegdehijhklij"
// pub fn partition_labels_2(s: String) -> Vec<i32> {
//     let mut cur = 0;
//     let mut res = vec![];
//     let mut len = 0;

//     for (mut idx, c) in s.chars().enumerate() {
//         let last = s.rfind(c).unwrap();
//         cur = std::cmp::max(cur, last);
//         len += 1;
//         if idx == cur {
//             res.push(len);
//             len = 0;
//         }
//         idx += 1;
//     }
//     res
}