pub fn exercise21(s1: String, s2: String) -> bool {
    let mut s2: Vec<char> = s2.chars().collect();
    for i in s1.chars() {
        for j in 0..s2.len() {
            if i == s2[j] {
                s2.remove(j);
                break;
            }
        }
    }
    s2.is_empty()
}

// pub fn is_anagram(input: Vec<&str>) -> bool {
//     if input[0].len() != input[1].len() {
//         return false;
//     }
//     let sum_1 = input[0].chars().fold(1, |mut acc, c| {
//         acc *= c as i64;
//         acc
//     });

//     let sum_2 = input[1].chars().fold(1, |mut acc, c| {
//         acc *= c as i64;
//         acc
//     });

//     sum_1 == sum_2
// }

// pub fn is_anagram_original(input: Vec<&str>) -> bool {
//    let mut sum_1 = 1;
//    let mut sum_2 = 1;

//    if input[0].len() != input[1].len() {
//       return false;
//    }

//    for w in input[0].chars() {
//       sum_1 *= w as i32;
//    }

//    for w in input[1].chars() {
//       sum_2 *= w as i32;
//    }
//    println!("sum1: {} sum2: {}", sum_1, sum_2);
//    sum_1 == sum_2
// }