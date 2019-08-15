pub fn title_to_number(input: String) -> i32 {
    const BASE: i32 = 26;
    const OFF_SET: i32 = 64;
    let letters: Vec<char> = input.chars().collect();

    letters
        .into_iter()
        .rev()
        .enumerate()
        .fold(0, |sum, (i, element)| {
            sum + (element as i32 - OFF_SET) * BASE.pow(i as u32)
        })
}