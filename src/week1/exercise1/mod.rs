pub fn array_by_parity(a: Vec<i32>) -> Vec<i32> {
    let mut evens: Vec<i32> = vec![];
    let mut odds: Vec<i32> = vec![];
    for n in &a {
        if n % 2 == 0 {
            println!("Even: {}", n);
            evens.push(*n);
        } else {
            println!("Odd: {}", n);
            odds.push(*n);
        }
    }
    [&evens[..], &odds[..]].concat()
}
