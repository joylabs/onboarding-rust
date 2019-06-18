pub fn self_dividing_numbers(left: i32, right: i32) -> Vec<i32> {
    let mut output: Vec<i32> = vec![];
    for number in left..=right {
        let number_string: Vec<char> = number.to_string().chars().collect();
        if number_string.len() > 1 {
            if check_number(number_string, number) {
                output.push(number);
            }
        } else {
            output.push(number);
        }
    }
    output
}

fn check_number(number_string: Vec<char>, number: i32) -> bool {
    let mut flag = false;
    for character in number_string {
        let character = character.to_digit(10).unwrap() as i32;
        if character != 0 && number % character == 0 {
            flag = true;
        } else {
            flag = false;
            break;
        }
    }
    flag
}
