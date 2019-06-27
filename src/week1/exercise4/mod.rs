pub fn self_dividing_numbers(left: i32, right: i32) -> Vec<i32> {
    let result: Vec<i32> = (left..=right)
        .filter(|n| check_number(*n))
        .collect::<Vec<i32>>();
    result
}

fn check_number(current_number: i32) -> bool {
    let number_string: Vec<char> = current_number.to_string().chars().collect();
    let mut flag = false;
    for character in number_string {
        let character = character.to_digit(10).unwrap() as i32;
        if character != 0 && current_number % character == 0 {
            flag = true;
        } else {
            flag = false;
            break;
        }
    }
    flag
}