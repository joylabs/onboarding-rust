pub fn self_dividing_numbers(input: Vec<i32>) -> Vec<i32> {
    (input[0]..=input[1]).filter(|a| is_div(*a)).collect()
}

fn is_div(i: i32) -> bool {
    let mut num = i;
    let mut is_div = true;

    while is_div && num > 0 {
        let num_module = num % 10;

        match num_module {
            0 => {
                is_div = false;
                break;
            }
            a if { i % a == 0 } => is_div = true,
            _ => is_div = false,
        }
        num /= 10;
    }
    is_div
}
