pub fn hamming_distance(input: Vec<i32>) -> i32 {
    let mut a = format!("{:b}", input[0]);
    let mut b = format!("{:b}", input[1]);
    let mut result = 0;

    if a.len() > b.len() {
        b.insert_str(0, &"0".repeat(a.len() - b.len()));
    } else if a.len() < b.len() {
        a.insert_str(0, &"0".repeat(b.len() - a.len()));
    }
    println!("{} {}", a, b);
    a.chars().zip(b.chars()).fold(0, |acc, (x, y)| {
        if x != y {
            return acc + 1;
        }
        acc
    })

    // for (x, y) in a.chars().zip(b.chars()) {

    //     if x != y {
    //         result += 1;
    //         println!("{} {}", x, y);
    //     }
    // }

    // result
}
