use std::collections::HashSet;

pub fn is_happy(n: i32) -> bool {
    let mut set: HashSet<i32> = HashSet::new();
    let mut number = sum_of_square_digits(n);

    while number != 1 {
        number = sum_of_square_digits(number);
        if !set.contains(&number) {
            set.insert(number);
        } else {
            break;
        }
    }

    number == 1
}

fn sum_of_square_digits(number: i32) -> i32 {
    let mut sum = (number % 10).pow(2);
    if number >= 10 {
        sum += sum_of_square_digits(number / 10);
    }
    sum
}
