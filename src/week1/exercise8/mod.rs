pub fn is_power_of_two(input: i32) -> bool {
    input >= 0 && input.count_ones() == 1
}
