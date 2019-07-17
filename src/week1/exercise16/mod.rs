pub fn hamming_distance(x: i32, y: i32) -> i32 {
    let mut difference = x ^ y;
    let mut count = 0;
    while difference > 0 {
        difference = difference & (difference - 1);
        count += 1;
    }
    count
}