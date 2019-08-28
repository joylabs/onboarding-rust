pub fn binary_gap(input: i32) -> i32 {

    if input.count_ones() == 1 || input.count_ones() == 0 {
        return 0;
    }
    let binary_num = format!("{:b}", input);
    let mut acc: i32 = 1;

    binary_num.chars().fold(1, |mut gap, num| {
        if num == '0' {
            acc += 1;
            gap
        } else if num == '1' {
            if acc >= gap {
                gap = acc;
            }
            acc = 1;
            gap
        } else {
            gap
        }
    })
}