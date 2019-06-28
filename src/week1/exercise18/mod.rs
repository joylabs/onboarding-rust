pub fn single_number(num: Vec<i32>) -> i32 {
    let mut input = num;

    input.sort();

    let mut unique = input.remove(0);
    let mut is_unique = 1;

    for comp in input {
        if unique == comp {
            is_unique += 1;
        } else {
            if is_unique == 1 {
                return unique;
            }
            is_unique = 1;
        }

        unique = comp;
    }

    unique
}
