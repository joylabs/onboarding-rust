pub fn single_number(num: Vec<i32>) -> i32 {
    let mut input = num;

    input.sort();

    let mut unique = input.remove(0);
    let mut is_unique = true;

    for next in input {
        if unique == next {
            is_unique = false;
        } else {
            if is_unique {
                return unique;
            }
            is_unique = true;
        }

        unique = next;
    }
    unique
}
