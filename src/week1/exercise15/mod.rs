pub fn number_complement(input: i32) -> i32 {
    let mut unos = u32::max_value();

    if input.leading_zeros() == 32 {
        return 1;
    }
    unos >>= input.leading_zeros();
    (input as u32 ^ unos) as i32
}

// SEGUNDA OPCION
// let binary_input = format!("{:b}", input);

// binary_input
//     .chars()
//     .rev()
//     .enumerate()
//     .fold(0, |acc, (j, i)| {
//         if i == '0' {
//             return acc + 2i32.pow(j as u32);
//         }
//         acc
//     })

// PRIMERA OPCION
// fn main() {
//     let x: i32 = 8;
//     let z = format!("{:b}", x);
//     let mut sum: i32 = 0;
//     let binary: i32 = 2;
//     for (j, i) in z.chars().rev().enumerate() {
//         if let '0' = i {
//             sum += binary.pow(j as u32);
//         }
//     }
// }
