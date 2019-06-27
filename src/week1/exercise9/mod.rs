pub fn reverse_string(input: &mut Vec<char>) {
    // Vec function that reverses the order of elements in the slice, in place.
    // input.reverse()
    let mut limit_left = 0;
    let mut limit_right = if !input.is_empty() {
        input.len() - 1
    } else {
        0
    };
    let mut character: char;
    while limit_right > limit_left {
        character = input[limit_right];
        input[limit_right] = input[limit_left];
        input[limit_left] = character;
        limit_left += 1;
        limit_right -= 1;
    }
}