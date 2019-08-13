pub fn self_dividing_numbers(left: i32, right: i32) -> Vec<i32> {
    (left..=right)
        .filter(|number| {
            number.to_string().chars().all(|element| {
                let digit = element.to_digit(10).unwrap() as i32;
                !(digit == 0 || number % digit != 0)
            })
        })
        .collect()
}