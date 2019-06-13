pub fn self_dividing_numbers(input: Vec<i32>) -> Vec<i32> {
    let mut output: Vec<i32> = vec![];

    for x in input[0]..=input[1] {
        let mut num = x;
        let mut is_div = true;

        while is_div && num > 0 {
            let num_module = num % 10;

            match num_module {
                0 => {
                    is_div = false;
                    break;
                }
                a if { x % a == 0 } => is_div = true,
                _ => is_div = false,
            }
            num /= 10;
        }

        if is_div {
            output.push(x);
        }
    }
    output
}