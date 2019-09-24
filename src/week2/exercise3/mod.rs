pub fn happy_numbers(n: i32) -> bool {
    let depth = 0;
    is_happy(n, depth)
}
fn is_happy(input: i32, depth: i64) -> bool {

    let sum_of_squares = input
        .to_string()
        .chars()
        .fold(0, |acc, x| acc + x.to_digit(10).unwrap().pow(2) as i32);

    if depth > 1000 {
        return false;
    } else if sum_of_squares == 1 {
        return true;
    }
    is_happy(sum_of_squares, depth + 1)
}

pub fn happy_numbers_2(n: i32) -> bool {
    let depth = 0;
    is_happy_2(n, depth)
}

fn is_happy_2(mut input: i32, mut depth: i64) -> bool {
    let mut total: i32 = 0;
    while input != 0 {
        total += (input % 10).pow(2);
        input /= 10;
    }
    input = total;
    depth += 1;
    if depth > 1000 {
        return false;
    } else if input == 1 {
        return true;
    }
    is_happy_2(input, depth)
}
