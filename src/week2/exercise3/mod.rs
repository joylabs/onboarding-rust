pub fn is_happy(n: i32) -> bool {
    let mut iterations = 0;
    let mut number = sum_of_square_digits(n);

    while (number != 1) && (iterations < 101) {
        number = sum_of_square_digits(number);
        iterations += 1;
    }

    number == 1
}

fn sum_of_square_digits(number: i32) -> i32 {
    number.to_string().chars().fold(0, |acc, n| {
        let digit = n.to_digit(10).unwrap() as i32;
        acc + (digit.pow(2))
    })
}
