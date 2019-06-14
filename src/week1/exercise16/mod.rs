//use itertools::EitherOrBoth::Both;

pub fn exercise16(x: i32, y: i32) -> i32 {
    if x > y {
        iterate(format!("{:b}", x), format!("{:b}", y))
    } else {
        iterate(format!("{:b}", y), format!("{:b}", x))
    }
}

fn iterate(s1: String, mut s2: String) -> i32 {
    let mut sum = 0;
    for i in s1.chars().rev() {
        let next_s2 = s2.pop();
        match i {
            i if i != '0' && next_s2 == None => sum += 1,
            i if next_s2 != None && i != next_s2.unwrap() => sum += 1,
            _ => (),
        }
    }
    sum
}

// pub fn Alex_suggestion(x: i32, y: i32) -> i32 {
//     let s1 = format!("{:b}", x);
//     let s2 = format!("{:b}", y);
//     s1.chars()
//         .rev()
//         .zip_longest(s2.chars().rev())
//         .map(|pair| match pair {
//             Both(i, j) if i == j => 0,
//             _ => 1,
//         })
//         .sum()
// }


// pub fn exercise16(x: i32, y: i32) -> i32 {
//     (x ^ y).count_ones() as i32
// }