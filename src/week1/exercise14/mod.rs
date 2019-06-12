pub fn hamming_distance(input: Vec<i32>) -> i32 {
    (input[0] ^ input[1]).count_ones() as i32

}
// OPTION 2
//     let mut a = format!("{:b}", input[0]);
//     let mut b = format!("{:b}", input[1]);

//     if a.len() > b.len() {
//         b.insert_str(0, &"0".repeat(a.len() - b.len()));
//     } else if a.len() < b.len() {
//         a.insert_str(0, &"0".repeat(b.len() - a.len()));
//     }
//     a.chars().zip(b.chars()).fold(0, |acc, (x, y)| {
//         if x != y {
//             return acc + 1;
//         }
//         acc
//     })
////////////////OPTION 1///////////////////////////////
//     let mut a = format!("{:b}", input[0]);
//     let mut b = format!("{:b}", input[1]);
//     let mut result = 0;

//     if a.len() > b.len() {
//         b.insert_str(0, &"0".repeat(a.len() - b.len()));
//     } else if a.len() < b.len() {
//         a.insert_str(0, &"0".repeat(b.len() - a.len()));
//     }

//     for (x, y) in a.chars().zip(b.chars()) {

//         if x != y {
//             result += 1;
//             println!("{} {}", x, y);
//         }
//     }

//     result